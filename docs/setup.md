# Jak zprovoznit Train Trouble

Tento dokument vás provede tím, jak spustit a provozovat hru Train Trouble.

## Spouštění v kontejneru

Nejjednodušší způsob jak spustit Train Trouble je pomocí kontejneru.
Stačí nainstalovat `podman` nebo `docker` a provést v kořeni repositáře pár příkazů:

```sh
# Sestavíme obraz:
podman build . --tag train-trouble

# Vytvoříme kontejner:
podman container create --publish 8000:8000 --name train-trouble train-trouble:latest

# Kontejner zpustíme:
podman container start train-trouble
```

Hotovo, hra běží na portu 8000!
Pro vypnutí je možné použít tento příkaz:

```sh
podman container stop train-trouble
```

Kontejner ukládá stav hry na disk, takže je možné jej vypnout,
znovu zapnout a pokračovat ve hře.
Je jen nutné pokaždé používat stejný kontejner a nevytvářet pokaždé nový.

## Spouštění bez kontejneru

Pokud nechcete používat kontejnery, tak musíte nejprve nainstalovat Rust a Node.js.
Pro spuštění serveru můžete použít tento příkaz v koření repositáře:

```sh
cargo run --release
```

Alternativně můžete pomocí tohoto příkazu server jen sestavit:

```sh
cargo build --release
```

Sestavenou binárku najdete v cestě `./target/release/train-trouble`.

Dále je potřeba sestavit front-end.
Ve složce `train-trouble-client/` spusťte tyto dva příkazy:

```sh
npm install
npm run build
```

Sestavený front-end najdete ve složce `train-trouble-client/dist/`.

Serveru můžete cestu k front-endu předat přepínačem `--serve`:

```sh
./target/release/train-trouble --serve train-trouble-client/dist/
# nebo
cargo run --release -- --serve train-trouble-client/dist/
```

Alternativně můžete pro front-end servírovat pomocí jiného HTTP serveru.
Stačí jej nastavit tak, aby požadavky na cestu `/api/sync`
přeposílala na server Train Trouble.
Je ale potřeba, aby tento server podporoval WebSockets.
