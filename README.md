# vkb-clicker

Autoclicker dla Linux/Wayland. Wysyla klikniecia myszy przez wirtualne
urzadzenie `/dev/uinput`, dzieki czemu dziala niezaleznie od kompozytora
(Wayland celowo blokuje aplikacjom nasluchiwanie globalnych skrotow
klawiszowych, ale nie blokuje wstrzykiwania zdarzen wejscia przez uinput).

Klikniecia trafiaja w biezaca pozycje kursora — appka nie rusza myszka,
tylko generuje wcisniecie/puszczenie przycisku.

## Budowanie

```sh
cargo build --release
```

Plik wynikowy: `target/release/vkb-clicker`. Mozna go skopiowac np. do
`~/.local/bin/vkb-clicker` (upewnij sie, ze ten katalog jest w `$PATH`).

## Uzycie

```sh
# start klikania: 20ms trzymania, 10ms przerwy (wartosci domyslne)
vkb-clicker --click-ms 20 --pause-ms 10

# lewy/prawy/srodkowy przycisk
vkb-clicker --click-ms 20 --pause-ms 10 --button left

# zatrzymanie dzialajacej instancji
vkb-clicker --kill
```

Program pilnuje jednej instancji na uzytkownika (plik PID w
`$XDG_RUNTIME_DIR/vkb-clicker.pid`) — ponowne uruchomienie bez `--kill`,
gdy klikanie juz trwa, zakonczy sie odmowa.

## Uprawnienia do /dev/uinput

Na tym systemie `/dev/uinput` nalezy do grupy `users` z prawem zapisu, wiec
zwykly uzytkownik moze z niego korzystac bez roota. Jesli na innej maszynie
dostaniesz `Permission denied`, dodaj regule udev, np.
`/etc/udev/rules.d/60-uinput.rules`:

```
KERNEL=="uinput", MODE="0660", GROUP="input", TAG+="uaccess"
```

i dodaj siebie do grupy `input` (`sudo usermod -aG input $USER`, potem
relogin).

## Podpiecie pod skroty klawiszowe (KDE Plasma)

Wayland nie pozwala aplikacjom przechwytywac globalnych skrotow, wiec to
kompozytor musi wywolac odpowiednia komende. W KDE Plasma:

1. **Ustawienia systemowe -> Skroty -> Wlasne skroty** (System Settings ->
   Shortcuts -> Custom Shortcuts).
2. Kliknij prawym na "Wlasne" -> Nowy -> Polecenie/Adres URL (Global
   Shortcut -> Command/URL).
3. Nazwa: `Start klikania`. Polecenie:
   `/home/bendyz/.local/bin/vkb-clicker --click-ms 20 --pause-ms 10`
   (podaj pelna sciezke do binarki). Przypisz np. `Ctrl+Alt+F9`.
4. Utworz drugi wpis: nazwa `Stop klikania`, polecenie
   `/home/bendyz/.local/bin/vkb-clicker --kill`, skrot np. `Ctrl+Alt+F10`.
5. Zastosuj. Od teraz pierwszy skrot startuje klikanie w miejscu kursora
   (np. nad przyciskiem w grze przegladarkowej), a drugi je zatrzymuje —
   dziala globalnie, niezaleznie od tego, ktore okno ma fokus.

## Uwaga

Automatyczne klikanie w grach przegladarkowych moze naruszac regulamin
danej gry/serwisu — to Twoja odpowiedzialnosc jako uzytkownika.
