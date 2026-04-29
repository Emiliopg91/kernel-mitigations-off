pkgname=kernel-mitigations-off
pkgver=1.1.2
pkgrel=1
pkgdesc='Pacman hook to disable CPU mitigations'
arch=('x86_64')
url='https://github.com/Emiliopg91/kernel-mitigations-off'
license=('GPL-2')
depends=(
  'pacman'
)
makedepends=(
  rust
)
source=(
  "git+$url.git#tag=$pkgver-$pkgrel"
)
sha256sums=(
  'SKIP'
)

build() {
  cd "$srcdir/$pkgname"
  cargo build --release
}

package() {
  install -Dm644 "$srcdir/$pkgname/hooks/06-kernel-mitigations-off.hook" "$pkgdir/usr/share/libalpm/hooks/06-kernel-mitigations-off.hook"
  install -Dm644 "$srcdir/$pkgname/hooks/zy-kernel-mitigations-off.hook" "$pkgdir/usr/share/libalpm/hooks/zy-kernel-mitigations-off.hook"
  install -Dm755 "$srcdir/$pkgname/target/release/kernel-mitigations-off" "$pkgdir/usr/share/libalpm/scripts/kernel-mitigations-off"
}