# AppimageMannger: hızlı başlangıç

AppimageMannger, Linux için yerel bir AppImage uygulama yöneticisidir. Soldaki listeden bir uygulama seçerek sağda simgesini, sürümünü, mimarisini, boyutunu ve kurulum yolunu görebilirsiniz. İçe aktarma, dosyayı yönetilen klasöre kopyalar; özgün indirmeyi silmez. Ayrıntılardan uygulamayı başlatabilir ve masaüstü menü kaydını yönetebilirsiniz.

## Kurulum ve açılış

Releases bölümünden işlemcinize uygun paketi indirin ve aşağıdaki yöntemlerden birini seçin. Taşınabilir sürüm için AppimageMannger.AppImage adını gerçek dosya adıyla değiştirin. Arch paketleri sağlanır; bu rehber resmî bir AUR kaydı bulunduğunu varsaymaz.

```sh
# Debian / Ubuntu
sudo apt install ./appimage-mannger*.deb
# Fedora / RPM
sudo dnf install ./appimage-mannger*.rpm
# Arch Linux
sudo pacman -U ./appimage-mannger*.pkg.tar.zst
# AppImage
chmod +x ./AppimageMannger.AppImage
./AppimageMannger.AppImage
```

## Kalıcı ayarlar

Varsayılan klasör ~/Applications konumudur. Ayarlardan değiştirebilirsiniz; mevcut uygulamalar otomatik taşınmaz. Gerçek GPUI/Ely, Qt 6 Widgets veya GNOME/GTK 4/libadwaita arayüzünü seçin. Arayüz değişikliği tercihleri kaydeder ve yöneticiyi yeniden başlatır. Tema, klasör, dil, seçili uygulama ve pencere durumu korunur. Ayarlar → Dil bölümünde 26 dilden birini seçebilirsiniz. Error, Info ve Warning tanılama düzeyleri bilerek İngilizce bırakılmıştır.

## Güncelleme ve kaldırma

Güncelleme denetiminden önce uygulamanın GitHub deposunu doğrulayın. Desteklenen .upd_info verileri okunur; owner/repository elle de girilebilir. Kararlı bir sürüm arayın ve uygun AppImage dosyasını yükleyin. Boyut ve biçim denetlenir; özet yayımlanmışsa SHA-256 doğrulanır. Değiştirme atomiktir ve önceki sürüm yedeklenir. Başarısız güncelleme kurulu uygulamayı korur.

Kaldırma, uygulamayı ve masaüstü kayıtlarını çöp kutusuna taşır. Bunları geri yükleyebilirsiniz; kişisel ayarlarınız korunur.

## Lisans ve kaynak

Proje AGPL-3.0-only kullanır. Kullanım, değiştirme ve dağıtım bu lisansın koşullarına bağlıdır. Dağıtım ve değiştirilmiş ağ hizmetleri, ilgili kaynak kodunu sağlama yükümlülükleri doğurur. Tam koşullar için LICENSE dosyasını okuyun.

[Kaynak ve sürümler](https://github.com/xingwangzhe/AppimageMannger)
