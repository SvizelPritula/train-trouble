# Train Trouble

Existuje pět lokací v terénu, které reprezentují nádraží.
Na každém z nich najdete QR kód, který obsahuje URL webové stránky,
přes kterou je možné hru ovládat.
URL adresy jsou následující:

| Nádraží             | URL                                     |
| ------------------- | --------------------------------------- |
| Nová Ves nad Spálou | http://localhost:8000/#nova-ves         |
| Kolnov              | http://localhost:8000/#kolnov           |
| Horní Mechoklaty    | http://localhost:8000/#horni-mechoklaty |
| Dolní Mechoklaty    | http://localhost:8000/#dolni-mechoklaty |
| Předvořany          | http://localhost:8000/#predvorany       |

Dále existuje základna, ve které projektor promítá živou mapu železnice,
nacházející se na adrese: http://localhost:8000/#map

Tyto URL adresy předpokládají, že server běží na portu 8000
na počítači, ze kterého k nim bude přistupováno.

## Železnice

Železnice se skládá z řady kolejí, které jsou vidět na mapě.
Koleje jsou jednosměrné.
Na kolejích se nachází výhybky,
které můžeme rozdělit na rozbočovací a spojovací.
Rozbočovací výhybky mohou být přehozené doleva nebo doprava.
Spojovací výhybky není třeba přehazovat.
Na kolejích se dále nachází návěstidla,
která mohou být v poloze volno nebo stůj.

Existuje pět nádraží,
každé návěstidlo nebo rozbočovací výhybka patří do nějakého z nich.
Je možné je ovládat přes webovou stránku daného nádraží.
Nádraží také mají jednu nebo dvě koleje.
Pozor, na nádraží není mapa železnice vidět.

Na železnici se také nachází dva vlaky.
Ty jedou neustále dopředu, ledaže přijedou k návěsti stůj,
u které se zastaví.
Koleje jsou rozdělené na bloky.
Na mapě jsou bloky, ve kterých se nachází vlak, označené zeleně.

Pokud se oba vlaky srazí, tak bude železnice na minutu zastavená,
poté se vlaky vrátí na jejich původní pozici.

## Obchod

Všichni hráči mají společný rozpočet, který začíná na 0 ₸.
Cílem hráčů je na konci hry mít rozpočet co nejvyšší.
Rozpočet může být kladný nebo záporný.

Obchoduje se s šesti různými komoditami.
Každé nádraží má svůj kurzovní lístek, který se během hry mění.
V něj je uvedená cena za kilo každé z surovin.

Pokud ve stanici na některé z jeho kolejí stojí vlak,
pak je možné komodity nakupovat nebo prodávat.
Stačí ve webovém rozhraní zvolit druh komodity,
zadat množství v kilech a kliknout na „Koupit“ nebo „Prodat“.
Koupené komodity budou ihned naloženy na daný vlak,
podobně prodané komodity budou z vlaku odstraněny.
Každý z vlaků má maximální kapacitu 100 kg.

Pokud se vlaky srazí, tak bude jejich náklad zničen
a z rozpočtu budou odečteny náklady na opravu ve výši 50 000 ₸.