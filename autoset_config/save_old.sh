#!/bin/bash

# Controllo del numero di argomenti
if [ $# -ne 3 ]; then
  echo "Uso: $0 <file_sorgente> <cartella_destinazione> <cartella_salva_vecchi>"
  exit 1
fi

# Assegnazione dei percorsi dalle variabili
file_sorgente="$1"
cartella_destinazione="$2"
cartella_salva_vecchi="$3"

# Controlla se il file sorgente esiste
if [ ! -f "$file_sorgente" ]; then
  echo "File sorgente '$file_sorgente' non trovato."
  exit 1
fi

# Controlla se la cartella di destinazione esiste
if [ ! -d "$cartella_destinazione" ]; then
  # Crea la cartella di destinazione se non esiste
  mkdir "$cartella_destinazione"
fi

# Controlla se la cartella per i file da salvare esiste
if [ ! -d "$cartella_salva_vecchi" ]; then
  # Crea la cartella per i file da salvare se non esiste
  mkdir "$cartella_salva_vecchi"
fi

# Ottieni il nome del file
nome_file=$(basename "$file_sorgente")

# Controlla se il file esiste già nella cartella di destinazione
if [ -f "$cartella_destinazione/$nome_file" ]; then
  # Sposta il file nella cartella per i file da salvare
  mv "$cartella_destinazione/$nome_file" "$cartella_salva_vecchi/$nome_file"
  echo "File '$nome_file' esistente spostato in '$cartella_salva_vecchi'"
else
  # Copia il file nella cartella di destinazione
  cp "$file_sorgente" "$cartella_destinazione/$nome_file"
  echo "File '$nome_file' copiato in '$cartella_destinazione'"
fi