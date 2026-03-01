if [[ $EUID -ne 0 ]]; then
   echo "אנא הרץ בהרשאות משתמש-על"
   exit 1
fi

if ! command -v cargo &> /dev/null; then
  echo "מוריד cargo";
    if command -v apt &> /dev/null; then
      sudo apt update && sudo apt install -y cargo;
    elif command -v pacman &> /dev/null; then
      sudo pacman -S --noconfirm cargo
    fi
    else
      echo "לא יכול לזהות distro"
      echo "אנא הורד cargo ידנית"
      exit 1

fi

echo "מקמפל אפליקציה..."
cargo build -q

cp ./target/debug/SHOE-ShoreshEditor /usr/bin/shoreshed

echo "וזהו!"
echo "השתמש ב shoreshed ואז את שם הקובץ שתרצה ליצור בכדי להתחיל לערוך!"