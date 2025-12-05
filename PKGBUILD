pkgname="trash_man"
pkgver=0.0.1
pkgrel=2
pkgdesc='Blazingly fast trash management written in rust'
arch=(x86_64)
makedepends=(
  cargo
)


build() {
    cargo build --release
}


package() {
    cd ..
    mkdir -p "${pkgdir}/etc/systemd/system"
    mkdir -p "${pkgdir}/usr/local/bin"
    cp "target/release/maid" "${pkgdir}/usr/local/bin/"
    cp "target/release/trash_man" "${pkgdir}/usr/local/bin/"
    cp "systemd/trash-man.service" "${pkgdir}/etc/systemd/system/"
    cp "systemd/trash-man.timer" "${pkgdir}/etc/systemd/system/"
}
