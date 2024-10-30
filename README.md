# Juicerss

<picture>
    <source srcset="https://brainmade.org/88x31-dark.png" media="(prefers-color-scheme: dark)">
    <img src="https://brainmade.org/88x31-light.png">
</picture>

[![Copr RPM build status](https://copr.fedorainfracloud.org/coprs/tulilirockz/juicerss/package/juicerss/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/tulilirockz/juicerss/package/juicerss/)

Read-only first RSS/Atom feed reader made for [Universal Blue](https://universal-blue.org) meant to be as simple and straight-forward as possible, and, 
as this is a changelog-first reader, it purposefully does not contain common features on RSS readers, like "read" articles, favorites, and others.

![Demo](./demo.gif)

Configuring should be done as [the example included in this repo](https://github.com/tulilirockz/juicerss/blob/main/example.toml)

## Installing

### cURL

If you are on any distro that does not have a package for this program, you can fetch the binary directly from releases

```
export VERSION=v1.0.3
wget -P $HOME/.local/bin https://github.com/tulilirockz/juicerss/releases/download/${VERSION}/juicerss-${VERSION}.x86_64
mv $HOME/.local/bin/juicerss-${VERSION}.x86_64 $HOME/.local/bin/juicerss
chmod +x $HOME/.local/bin/juicerss
# Then make sure to have $HOME/.local/bin in your PATH variable
```

### Fedora

On Fedora / RPM-based distros you can install this by using the supplied [COPR](https://copr.fedorainfracloud.org/coprs/tulilirockz/juicerss)

```
sudo dnf copr enable tulilirockz/juicerss
sudo dnf install -y juicerss
```

## Todo

- Mouse support
- Highlighting for Markdown

## Sponsoring

This project is made by a Human Being! If you want to help me out doing these kinds of projects in the future, please 
[Sponsor me](https://github.com/sponsors/tulilirockz) on Github. It would be much appreciated and it'll help me with equipment and tons of stuff to continue 
making good software.
