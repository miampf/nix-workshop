---
title: Nix workshop
description: Workshop zum nix lernen :)
paginate: true
marp: true
theme: gaia
---

<!-- _class: lead -->

# Nix Workshop

![bg center](https://raw.githubusercontent.com/NixOS/nixos-artwork/refs/heads/master/logo/nix-snowflake-colours.svg)

---

# Warum nix?

1. Umgebungsverwaltung
   - Keine nervigen globalen Umgebungen
2. Reproduzierbarkeit
   - "It works on my machine" ist keine Ausrede mehr
   - Immer und überall dasselbe Ergebnis
3. Verwaltbarkeit
   - Weniger Probleme $\rightarrow$ weniger Aufwand :)

---

# Und wie überhaupt?

```nix
  {stdenv, fetchurl, ...}: {}
```

---

```nix
{stdenv, fetchurl, ...}:

stdenv.mkDerivation {}
```

---

```nix
{stdenv, fetchurl, ...}:

stdenv.mkDerivation {
    pname = "foobar";
    version = "0.1.0";
    src = fetchurl {
      url = "...";
      sha256 = "...";
    };
}
```

---

# Yay, Schneeflocken

- Flakes sind _theoretisch_ noch experimentell
- haben `inputs` und `outputs`

---

```nix
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  # ...
}
```

---

```nix
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in
  {
    # ...
  };
}
```

---

```nix
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in
  {

    packages.${system}.hello = pkgs.hello;

    packages.${system}.default = self.packages.${system}.hello;

  };
}
```

---

# Audience Participation!

- https://nixos.org/download/ oder `docker pull nixos/nix`
- Für Windows-Leute: https://wiki.nixos.org/wiki/WSL

---

Flakes aktivieren:

```sh
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

---

Dann repo clonen und zur Demo wechseln:

```sh
git clone https://github.com/miampf/nix-workshop
cd nix-workshop/demo-program
```

---

# Sachen finden

- Nix ist leider nicht das am besten dokumentierte System :(
- Source code lesen!
- Zum Sachen nachschlagen:
  - https://search.nixos.org/packages
  - https://wiki.nixos.org
  - https://noogle.dev
  - https://nix.dev
  - https://nixos.org/manual/nixos/stable/


---

# Selbstständiges Arbeiten >:)

![bg center](https://funnicons.com/cdn/shop/products/thumbupemojipreview01.jpg?v=1658746316)
