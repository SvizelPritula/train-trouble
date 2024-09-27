# Train Trouble Engine

Hra Train Trouble používá vlastní engine, který byl navržený tak,
aby se dal použít i pro jiné, podobné hry.
Řeší veškerou komunikaci mezi klienty a serverem.

## Kanály, pohledy a akce

Hlavními koncepty enginu jsou kanály (channel), pohledy (view) a akce (action).

Každý klient je připojen k nějakému kanálu.
Kanál udává, co všechno klient vidí a co může dělat.

Pro každý kanál server průběžně počítá pohled.
Pokaždé, když se změní, tak jej rozešle všem klientům v daném kanálu.
Klient pak pravděpodobně na základě jej změní uživatelské rozhraní.

Nakonec může klient poslat na server akci.
Akce patří to kanálu, ke kterému je připojen.
Server tuto akci zpracuje a klientovi ji buď potvrdí,
nebo zamítne s libovolnou chybovou hláškou.

Kanálu, pohledy a akce mohou být libovolné JSONové hodnoty.
Na straně serveru se parsují pomocí knihovny `serde`.

## Implementace serveru

Server musí vytvořit typ, který implementuje následující trait:

```rust
pub trait Game: Serialize + for<'de> Deserialize<'de> + Clone + Default + Send + 'static {
    type CHANNEL: for<'de> Deserialize<'de> + Clone + Eq + Hash + Send + Sync;
    type VIEW: Serialize + Clone + Eq + Send + Sync;
    type ACTION: for<'de> Deserialize<'de> + Clone + Eq + Send + Sync;

    const TICK_RATE: u64;

    fn tick(&mut self);
    fn view(&mut self, channel: Self::CHANNEL) -> Self::VIEW;
    fn act(&mut self, channel: Self::CHANNEL, action: Self::ACTION) -> ActionResult;
}
```

Je dále potřeba vytvořit typy pro kanál, pohled a akci.
Všechny typy musí implementovat hromadu dalších traitů pro serializaci, kopírování, porovnávání apod.
Všechny tyto implementace je možné vytvořit automaticky pomocí `#[derive(...)]`.
Také je potřeba určit konstantu `TICK_RATE`.

Nejjednodušší je funkce `tick`, která bude volána `TICK_RATE`-krát za sekundu.
Je určená pro provádění potřebných simulací.

Další funkce je `view`.
Ta bude po zavolána po každém zavolání `tick` pro každý kanál,
ke kterému je někdo připojený.
Pro tento kanál má za úkol spočítat aktuální pohled.

Poslední funkcí je `act`.
Ta je zavolána pokaždé, když nějaký klient odešle akci.
V ní je potřeba na tuto akci zareagovat.
Pokud akce proběhla bez problémů, tak by tato funkce měla vrátit hodnotu `ActionResult::Ok`.
Pokud nastala chyba, pak by měla vrátit `ActionResult::Error(Box<str>)` s chybovou hláškou.
Pokud se jedná o akci, kterou v tomto kanálu nelze provést,
tak by měla vrátit hodnotu `ActionResult::Misdirected`.

Vždy je provedeno nula nebo více zavolání `act`,
poté jedno zavolání `tick` a nakonec nula nebo více zavolání `view`,
a takto donekonečna dokola.

## Implementace klienta

Klient má k dispozici tuto TypeScriptovou funkci:

```ts
function connect<Channel, View, Action>(
  channel: Channel,
  onNewView: (view: View) => void,
  onConnectionChange: (connected: boolean) => void
): Connection<Action>;

interface Connection<Action> {
  stop(): void,
  submit(action: Action): Promise<void>,
}
```

Tato funkce vytvoří spojení k serveru.
Tuto funkci stačí zavolat jednou,
v případě přerušení spojení se pokusí periodicky spojení automaticky obnovit.
Je opět potřeba nejprve vytvořit typy pro kanál, pohled a akci.
Tyto typy jsou generickými parametry této funkce.

Prvním argumentem `channel` je kanál, ke kterému se klient chce připojit.
Druhým argumentem `onNewView` je funkce, která bude zavolána pokaždé,
když od serveru přijde nový pohled.
Tento pohled bude jejím argumentem.
Třetím argumentem je funkce `onConnectionChange`.
Po připojení k serveru bude zavolána s argumentem `true`,
po (byť jen dočasném) odpojení bude zavolána s argumentem `false`.

Funkce `connect` vrátí objekt s dvěma funkcemi.
Funkce `stop` je jednoduchá, spojení k serveru uzavře
(a vypne automatické znovupřipojování).
Asynchronní funkce `submit` odešle akci, kterou dostane jako argument.
Doběhne poté, co je akce provedena a potvrzena serverem.
Pokud nastane nějaká chyba, tak vyhodí výjimku `SubmitError`.
Vlastnost `reason` popisuje, jaká chyba nastala.
Pokud je `reason` `"rejected"`, pak se jedná o chybu ohlášenou funkcí `act` na serveru.
Chybová hláška pak bude uložená ve vlastnosti `message`.
