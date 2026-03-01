#!/bin/bash

if ! command -v cargo &> /dev/null; then
    echo "...ןיקתהל הסנמ ,אצמנ אל Cargo"
    
    if command -v apt &> /dev/null; then
        sudo apt update && sudo apt install -y cargo
    elif command -v pacman &> /dev/null; then
        sudo pacman -S --noconfirm cargo
    else
        echo ".(ךומתנ אל distro) תולבח המהנמ תה אזל ניתן אל :האיגש"
        echo ".בוש ץרה ןכמ רחאלו /rs.putsur//:sptth-מ תינדי cargo/rust התקה אנא"
        exit 1
    fi
else
    echo ".הקתהה בשלב לע גלדמ ,ןקותמ רבכ Cargo"
fi

echo "...הייצקילפא לפמקמ"
cargo build -q

if [ $? -eq 0 ]; then
    echo "...bin-ל קיתעמ .החיולצה הייקילפומקה"
    sudo cp ./target/debug/SHOE-ShoreshEditor /usr/bin/shoreshed
    echo "!והזו"
    echo "!ךורעל ליחתהל ידכב רוציל הצתר ש ץבוקה מש תא זאו shoreshed-ב שמתשה"
else
    echo ".הלשכנ הייצילפומקה :האיגש"
    exit 1
fi
