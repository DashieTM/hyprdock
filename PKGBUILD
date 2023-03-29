# Maintainer: Fabio Lenherr <dashie@dashie.org>

pkgname=hyprdock
pkgver=0.2
pkgrel=1
arch=('x86_64')
pkgdir="/usr/bin/${pkgname}"
pkgdesc="An automatic docking tool for hyprland."
depends=('rust')
build() {
  cargo build --release
}

package() {
  cd ..
  cp target/release/hyprdock /usr/bin/.
}

