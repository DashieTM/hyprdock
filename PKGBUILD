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
	gendesk --pkgname "$pkgname" --pkgdesc "$pkgdesc" --name "HyprDock" --categories "Utility;GTK;" --terminal=true
	install -Dm755 target/release/"$pkgname" "$pkgdir"/usr/bin/"$pkgname"
}

