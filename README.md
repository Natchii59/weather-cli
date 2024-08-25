# Weather CLI

Weather CLI est une application en ligne de commande (CLI) développée en Rust qui permet d'obtenir la météo actuelle pour une ville spécifiée, en utilisant l'API OpenWeatherMap.

## Fonctionnalités

- Récupère les informations météo pour une ville donnée via une requête à l'API OpenWeatherMap.
- Affiche la température, l'humidité, la pression, la vitesse du vent et une description générale du temps.
- Colore les informations météo en fonction de la description du temps (par exemple, ciel clair, pluie, etc.).
- Emoji qui illustre la température actuelle.
- Gestion des erreurs, notamment lorsque la ville n'est pas trouvée ou que la clé API est incorrecte.

## Prérequis

Avant d'exécuter ce projet, assurez-vous d'avoir installé les éléments suivants :

- [Rust](https://www.rust-lang.org/tools/install)
- Une clé API de [OpenWeatherMap](https://home.openweathermap.org/users/sign_up)

## Installation

1. Clonez ce dépôt sur votre machine locale :

```bash
git clone https://github.com/Natchii59/weather-cli.git
cd weather-cli
```

2. Installez les dépendances Rust nécessaires :

Le fichier Cargo.toml inclut les dépendances suivantes :

- `reqwest`
- `serde`
- `colored`
- `dotenv`

Les dépendances seront installées automatiquement lorsque vous compilerez le projet.

3. Créez un fichier .env à la racine du projet pour stocker votre clé API :

```bash
cp .env.sample .env
```

Ajoutez votre clé API dans ce fichier sous la forme suivante :

```bash
API_KEY=YOUR_API_KEY
```

4. Compilez le projet :

```bash
cargo build --release
```

## Utilisation

Pour exécuter l'application, utilisez la commande suivante :

```bash
cargo run --release
```

L'application vous demandera de saisir le nom d'une ville et le code de pays (par exemple, "FR" pour la France). Elle affichera ensuite les informations météo pour cette ville.

### Exemple de sortie

```
Welcome to Weather CLI!
Please enter the name of the city:
Paris
Please enter the country code (e.g., US for United States):
FR
Weather in Paris: clear sky 🌤️
> Temperature: 25.3°C
> Humidity: 45.0%
> Pressure: 1015.0 hPa
> Wind Speed: 3.5 m/s
Do you want to search for weather in another city? (yes/no):
```

## Structure du projet

- `main.rs` : Point d'entrée de l'application, gère la logique principale et le flux du programme.
- `weather.rs` : Contient les fonctions pour récupérer et afficher les informations météo.
- `utils.rs` : Contient les fonctions utilitaires, telles que la gestion de l'entrée utilisateur.
