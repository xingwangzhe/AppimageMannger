#!/usr/bin/env bun
/** Localized README bodies reuse the maintained QuickStarts; screenshots stay English. */
import {readFileSync, writeFileSync} from 'node:fs';
const locales: [string,string][] = [['en','English'],['zh-CN','简体中文'],['zh-TW','繁體中文'],['ja','日本語'],['ko','한국어'],['de','Deutsch'],['fr','Français'],['es','Español'],['pt-BR','Português (Brasil)'],['ru','Русский'],['it','Italiano'],['nl','Nederlands'],['pl','Polski'],['tr','Türkçe'],['uk','Українська'],['vi','Tiếng Việt'],['id','Bahasa Indonesia'],['th','ไทย'],['ar','العربية'],['hi','हिन्दी'],['bn','বাংলা'],['fa','فارسی'],['he','עברית'],['sv','Svenska'],['cs','Čeština'],['ro','Română']];
type Copy = {screens:string; note:string; features:string; bullets:string[]; develop:string};
const copy:Record<string,Copy> = {
 en:{screens:'Three native interfaces',note:'All screenshots show the English interface of version 1.0.1. Every translated README uses the same three screenshots.',features:'Features',develop:'Development',bullets:[
 'Import replaces older versions; original downloads remain, and previous installations can be restored from Trash.',
 'Application icons and AppImage Open With integration; `--repair-registration` restores registration.',
 'Anonymous HTTPS Release downloads with architecture and SHA-256 checks, real progress, percentage and transferred size. No `gh` or token is required.',
 'Draggable dividers and saved preferences across Ely/GPUI, Qt 6 Widgets and GNOME/GTK 4, with display names localized into 26 languages.',
 'Read application metadata and icons without running or mounting the AppImage.']},
 'zh-CN':{screens:'三种原生界面',note:'截图统一使用 1.0.1 的英文界面。所有语言版本的 README 共用以下三张截图。',features:'主要功能',develop:'开发',bullets:[
 '导入新版本时覆盖已安装版本；保留原始下载文件，旧版可从回收站恢复。',
 '应用图标和 AppImage“打开方式”集成；可用 `--repair-registration` 修复注册。',
 '匿名 HTTPS Release 直接下载，检查架构和 SHA-256，显示真实进度、百分比及已下载大小，无需 `gh` 或 token。',
 'Ely/GPUI、Qt 6 Widgets、GNOME/GTK 4 均支持拖动分隔线和保存偏好，应用显示名称覆盖 26 种语言。',
 '读取应用元数据和图标时不会运行或挂载 AppImage。']},
 'zh-TW':{screens:'三種原生介面',note:'截圖統一使用 1.0.1 的英文介面。所有語言版本的 README 共用以下三張截圖。',features:'主要功能',develop:'開發',bullets:[
 '匯入新版本時覆蓋已安裝版本；保留原始下載檔，舊版可從垃圾桶復原。',
 '應用程式圖示與 AppImage「開啟方式」整合；可用 `--repair-registration` 修復註冊。',
 '匿名 HTTPS Release 直接下載，檢查架構與 SHA-256，顯示實際進度、百分比及已下載大小，無需 `gh` 或 token。',
 'Ely/GPUI、Qt 6 Widgets、GNOME/GTK 4 均支援拖動分隔線及儲存偏好，應用程式名稱提供 26 種語言。',
 '讀取中繼資料與圖示時不會執行或掛載 AppImage。']},
 ja:{screens:'3 種類のネイティブ UI',note:'画像はすべてバージョン 1.0.1 の英語 UI です。各言語の README で同じ 3 枚を使用しています。',features:'主な機能',develop:'開発',bullets:[
 '新しいバージョンのインポートで既存版を置き換えます。元のダウンロードは残り、旧版はゴミ箱から復元できます。',
 'アプリアイコンと AppImage の「別のアプリで開く」を統合。`--repair-registration` で登録を修復できます。',
 '匿名 HTTPS で Release を直接取得。アーキテクチャと SHA-256 を検証し、進捗、割合、転送量を表示します。`gh` やトークンは不要です。',
 'Ely/GPUI、Qt 6 Widgets、GNOME/GTK 4 で区切り線をドラッグでき、設定を保存します。表示名は 26 言語に対応します。',
 'AppImage を実行・マウントせずにメタデータとアイコンを読み取ります。']},
 ko:{screens:'세 가지 네이티브 인터페이스',note:'모든 스크린샷은 1.0.1 버전의 영어 인터페이스입니다. 각 언어 README는 동일한 세 이미지를 사용합니다.',features:'주요 기능',develop:'개발',bullets:[
 '새 버전을 가져오면 기존 버전을 교체합니다. 원본 다운로드는 유지하며 이전 버전은 휴지통에서 복원할 수 있습니다.',
 '앱 아이콘과 AppImage의 연결 프로그램을 등록합니다. `--repair-registration`으로 등록을 복구할 수 있습니다.',
 '익명 HTTPS로 Release를 직접 다운로드하고 아키텍처와 SHA-256을 확인합니다. 실제 진행률과 전송량을 표시하며 `gh`나 토큰이 필요 없습니다.',
 'Ely/GPUI, Qt 6 Widgets, GNOME/GTK 4에서 구분선을 드래그하고 설정을 저장합니다. 앱 표시 이름은 26개 언어를 지원합니다.',
 'AppImage를 실행하거나 마운트하지 않고 메타데이터와 아이콘을 읽습니다.']},
 de:{screens:'Drei native Oberflächen',note:'Alle Bilder zeigen die englische Oberfläche von Version 1.0.1. Jede übersetzte README verwendet dieselben drei Bilder.',features:'Funktionen',develop:'Entwicklung',bullets:[
 'Der Import ersetzt ältere Versionen. Die ursprüngliche Datei bleibt erhalten; vorherige Installationen lassen sich aus dem Papierkorb wiederherstellen.',
 'Anwendungssymbole und AppImage-Integration für „Öffnen mit“. `--repair-registration` repariert die Registrierung.',
 'Anonyme HTTPS-Downloads von Releases mit Architektur- und SHA-256-Prüfung sowie echtem Fortschritt, Prozentwert und Datenmenge. Kein `gh` oder Token erforderlich.',
 'Verschiebbare Trennlinien und gespeicherte Einstellungen in Ely/GPUI, Qt 6 Widgets und GNOME/GTK 4; Anzeigenamen in 26 Sprachen.',
 'Metadaten und Symbole werden gelesen, ohne das AppImage auszuführen oder einzuhängen.']},
 fr:{screens:'Trois interfaces natives',note:'Les captures montrent toutes l’interface anglaise de la version 1.0.1. Chaque README traduit utilise les trois mêmes images.',features:'Fonctionnalités',develop:'Développement',bullets:[
 'L’import remplace les anciennes versions. Le téléchargement d’origine est conservé et les versions précédentes restent récupérables dans la corbeille.',
 'Icônes et intégration AppImage dans « Ouvrir avec ». `--repair-registration` répare l’enregistrement.',
 'Téléchargement direct des Releases par HTTPS anonyme, vérification de l’architecture et de SHA-256, progression réelle, pourcentage et volume transféré. Aucun `gh` ni jeton requis.',
 'Séparateurs déplaçables et préférences enregistrées avec Ely/GPUI, Qt 6 Widgets et GNOME/GTK 4 ; noms affichés dans 26 langues.',
 'Lecture des métadonnées et des icônes sans exécuter ni monter l’AppImage.']},
 es:{screens:'Tres interfaces nativas',note:'Todas las capturas muestran la interfaz en inglés de la versión 1.0.1. Los README traducidos usan las mismas tres imágenes.',features:'Funciones',develop:'Desarrollo',bullets:[
 'La importación sustituye las versiones anteriores. Conserva la descarga original y permite recuperar las instalaciones anteriores desde la papelera.',
 'Iconos e integración de AppImage en «Abrir con». `--repair-registration` repara el registro.',
 'Descargas directas de Releases por HTTPS anónimo, comprobación de arquitectura y SHA-256, progreso real, porcentaje y tamaño transferido. No requiere `gh` ni token.',
 'Separadores ajustables y preferencias guardadas en Ely/GPUI, Qt 6 Widgets y GNOME/GTK 4; nombres visibles en 26 idiomas.',
 'Lee metadatos e iconos sin ejecutar ni montar el AppImage.']},
 'pt-BR':{screens:'Três interfaces nativas',note:'Todas as capturas mostram a interface em inglês da versão 1.0.1. Cada README traduzido usa as mesmas três imagens.',features:'Recursos',develop:'Desenvolvimento',bullets:[
 'A importação substitui versões anteriores. O download original é preservado e instalações antigas podem ser restauradas da lixeira.',
 'Ícones e integração de AppImage em “Abrir com”. `--repair-registration` repara o registro.',
 'Downloads diretos de Releases por HTTPS anônimo, com verificação de arquitetura e SHA-256, progresso real, porcentagem e volume transferido. Não exige `gh` nem token.',
 'Divisórias ajustáveis e preferências salvas em Ely/GPUI, Qt 6 Widgets e GNOME/GTK 4; nomes de exibição em 26 idiomas.',
 'Lê metadados e ícones sem executar nem montar o AppImage.']},
 ru:{screens:'Три нативных интерфейса',note:'Все снимки показывают английский интерфейс версии 1.0.1. Во всех переводах README используются те же три изображения.',features:'Возможности',develop:'Разработка',bullets:[
 'Импорт заменяет старые версии. Исходный файл сохраняется, а предыдущие установки можно восстановить из корзины.',
 'Значки приложений и интеграция AppImage в меню «Открыть с помощью». Команда `--repair-registration` восстанавливает регистрацию.',
 'Прямая анонимная загрузка Releases по HTTPS с проверкой архитектуры и SHA-256, реальным прогрессом, процентом и объёмом передачи. `gh` и токен не нужны.',
 'Перетаскиваемые разделители и сохранение настроек в Ely/GPUI, Qt 6 Widgets и GNOME/GTK 4; названия приложения на 26 языках.',
 'Чтение метаданных и значков без запуска или монтирования AppImage.']},
 it:{screens:'Tre interfacce native',note:'Tutte le immagini mostrano l’interfaccia inglese della versione 1.0.1. Ogni README tradotto usa le stesse tre immagini.',features:'Funzionalità',develop:'Sviluppo',bullets:[
 'L’importazione sostituisce le versioni precedenti. Il download originale resta disponibile e le vecchie installazioni si possono recuperare dal cestino.',
 'Icone e integrazione AppImage in «Apri con». `--repair-registration` ripara la registrazione.',
 'Download diretti delle Release tramite HTTPS anonimo, controlli di architettura e SHA-256, avanzamento reale, percentuale e dati trasferiti. Non servono `gh` né token.',
 'Divisori trascinabili e preferenze salvate in Ely/GPUI, Qt 6 Widgets e GNOME/GTK 4; nomi visualizzati in 26 lingue.',
 'Lettura di metadati e icone senza eseguire o montare l’AppImage.']},
 nl:{screens:'Drie native interfaces',note:'Alle afbeeldingen tonen de Engelse interface van versie 1.0.1. Elke vertaalde README gebruikt dezelfde drie afbeeldingen.',features:'Functies',develop:'Ontwikkeling',bullets:[
 'Importeren vervangt oudere versies. De oorspronkelijke download blijft behouden; eerdere installaties zijn vanuit de prullenbak te herstellen.',
 'Toepassingspictogrammen en AppImage-integratie in ‘Openen met’. `--repair-registration` herstelt de registratie.',
 'Releases rechtstreeks via anonieme HTTPS downloaden, met controle van architectuur en SHA-256, echte voortgang, percentage en overgedragen omvang. Geen `gh` of token nodig.',
 'Versleepbare scheidingen en opgeslagen voorkeuren in Ely/GPUI, Qt 6 Widgets en GNOME/GTK 4; weergavenamen in 26 talen.',
 'Metadata en pictogrammen lezen zonder het AppImage uit te voeren of te koppelen.']},
 pl:{screens:'Trzy natywne interfejsy',note:'Wszystkie zrzuty pokazują angielski interfejs wersji 1.0.1. Każdy przetłumaczony README używa tych samych trzech obrazów.',features:'Funkcje',develop:'Rozwój',bullets:[
 'Import zastępuje starsze wersje. Oryginalny plik pozostaje, a poprzednie instalacje można przywrócić z kosza.',
 'Ikony aplikacji i integracja AppImage z „Otwórz za pomocą”. `--repair-registration` naprawia rejestrację.',
 'Bezpośrednie anonimowe pobieranie Releases przez HTTPS, kontrola architektury i SHA-256, rzeczywisty postęp, procent i rozmiar przesłanych danych. Bez `gh` i tokena.',
 'Przeciągane podziały i zapis ustawień w Ely/GPUI, Qt 6 Widgets oraz GNOME/GTK 4; nazwy wyświetlane w 26 językach.',
 'Odczyt metadanych i ikon bez uruchamiania ani montowania AppImage.']},
 tr:{screens:'Üç yerel arayüz',note:'Tüm ekran görüntüleri 1.0.1 sürümünün İngilizce arayüzünü gösterir. Çevrilmiş README dosyaları aynı üç görüntüyü kullanır.',features:'Özellikler',develop:'Geliştirme',bullets:[
 'İçe aktarma eski sürümün yerini alır. Özgün indirme korunur; önceki kurulumlar çöp kutusundan geri yüklenebilir.',
 'Uygulama simgeleri ve AppImage için “Birlikte aç” bütünleşmesi. `--repair-registration` kaydı onarır.',
 'Mimari ve SHA-256 denetimiyle anonim HTTPS üzerinden doğrudan Release indirme; gerçek ilerleme, yüzde ve aktarılan boyut. `gh` veya belirteç gerekmez.',
 'Ely/GPUI, Qt 6 Widgets ve GNOME/GTK 4 üzerinde sürüklenebilir ayırıcılar ve kalıcı tercihler; 26 dilde uygulama adı.',
 'AppImage dosyasını çalıştırmadan veya bağlamadan üst verileri ve simgeleri okur.']},
 uk:{screens:'Три нативні інтерфейси',note:'Усі знімки показують англійський інтерфейс версії 1.0.1. Кожен переклад README використовує ті самі три зображення.',features:'Можливості',develop:'Розробка',bullets:[
 'Імпорт замінює старі версії. Початковий файл зберігається, а попередні встановлення можна відновити з кошика.',
 'Піктограми й інтеграція AppImage у «Відкрити за допомогою». `--repair-registration` відновлює реєстрацію.',
 'Пряме анонімне завантаження Releases через HTTPS, перевірка архітектури й SHA-256, справжній поступ, відсоток та обсяг переданих даних. Без `gh` і токена.',
 'Рухомі роздільники та збережені налаштування в Ely/GPUI, Qt 6 Widgets і GNOME/GTK 4; назви 26 мовами.',
 'Читання метаданих і піктограм без запуску або монтування AppImage.']},
 vi:{screens:'Ba giao diện gốc',note:'Tất cả ảnh chụp đều dùng giao diện tiếng Anh của phiên bản 1.0.1. Các README đã dịch dùng chung ba ảnh này.',features:'Tính năng',develop:'Phát triển',bullets:[
 'Nhập phiên bản mới sẽ thay phiên bản cũ. Tệp tải gốc được giữ lại; bản cũ có thể khôi phục từ thùng rác.',
 'Biểu tượng ứng dụng và tích hợp AppImage trong “Mở bằng”. `--repair-registration` sửa đăng ký.',
 'Tải trực tiếp Release qua HTTPS ẩn danh, kiểm tra kiến trúc và SHA-256, hiển thị tiến độ thực, phần trăm và dung lượng đã tải. Không cần `gh` hoặc token.',
 'Kéo đường chia và lưu tùy chọn trong Ely/GPUI, Qt 6 Widgets và GNOME/GTK 4; tên hiển thị được dịch sang 26 ngôn ngữ.',
 'Đọc siêu dữ liệu và biểu tượng mà không chạy hay gắn kết AppImage.']},
 id:{screens:'Tiga antarmuka native',note:'Semua tangkapan layar menggunakan antarmuka bahasa Inggris versi 1.0.1. Setiap README terjemahan memakai tiga gambar yang sama.',features:'Fitur',develop:'Pengembangan',bullets:[
 'Impor mengganti versi lama. Unduhan asli tetap disimpan; instalasi sebelumnya dapat dipulihkan dari tempat sampah.',
 'Ikon aplikasi dan integrasi AppImage pada “Buka dengan”. `--repair-registration` memperbaiki pendaftaran.',
 'Unduh Release langsung melalui HTTPS anonim, dengan pemeriksaan arsitektur dan SHA-256, progres nyata, persentase, serta ukuran transfer. Tidak memerlukan `gh` atau token.',
 'Pemisah yang dapat diseret dan preferensi tersimpan pada Ely/GPUI, Qt 6 Widgets, serta GNOME/GTK 4; nama tampilan dalam 26 bahasa.',
 'Membaca metadata dan ikon tanpa menjalankan atau memasang AppImage.']},
 th:{screens:'อินเทอร์เฟซเนทีฟสามแบบ',note:'ภาพทั้งหมดแสดงอินเทอร์เฟซภาษาอังกฤษของรุ่น 1.0.1 โดย README ทุกภาษาใช้ภาพสามภาพเดียวกันนี้',features:'คุณสมบัติ',develop:'การพัฒนา',bullets:[
 'การนำเข้าจะแทนที่รุ่นเก่า โดยเก็บไฟล์ดาวน์โหลดต้นฉบับไว้ และกู้คืนการติดตั้งเดิมจากถังขยะได้',
 'รองรับไอคอนแอปและเมนู “เปิดด้วย” สำหรับ AppImage ใช้ `--repair-registration` เพื่อซ่อมการลงทะเบียน',
 'ดาวน์โหลด Release โดยตรงผ่าน HTTPS แบบไม่ระบุตัวตน ตรวจสอบสถาปัตยกรรมและ SHA-256 พร้อมแสดงความคืบหน้า เปอร์เซ็นต์ และขนาดที่ดาวน์โหลดจริง ไม่ต้องมี `gh` หรือโทเคน',
 'ลากเส้นแบ่งและบันทึกค่ากำหนดใน Ely/GPUI, Qt 6 Widgets และ GNOME/GTK 4 พร้อมชื่อแอปที่แปลเป็น 26 ภาษา',
 'อ่านข้อมูลกำกับและไอคอนโดยไม่เรียกใช้หรือเมานต์ AppImage']},
 ar:{screens:'ثلاث واجهات أصلية',note:'تُظهر جميع الصور الواجهة الإنجليزية للإصدار 1.0.1. تستخدم كل ترجمات README الصور الثلاث نفسها.',features:'الميزات',develop:'التطوير',bullets:[
 'يستبدل الاستيراد الإصدارات الأقدم مع إبقاء التنزيل الأصلي، ويمكن استعادة التثبيت السابق من سلة المهملات.',
 'أيقونات التطبيقات وتكامل AppImage مع «فتح باستخدام». يعيد `--repair-registration` إصلاح التسجيل.',
 'تنزيل Releases مباشرة عبر HTTPS مجهول الهوية مع فحص المعمارية وSHA-256 وعرض التقدم الفعلي والنسبة والحجم المنقول. لا حاجة إلى `gh` أو رمز وصول.',
 'فواصل قابلة للسحب وتفضيلات محفوظة عبر Ely/GPUI وQt 6 Widgets وGNOME/GTK 4، مع أسماء عرض مترجمة إلى 26 لغة.',
 'قراءة البيانات الوصفية والأيقونات دون تشغيل AppImage أو وصله كنظام ملفات.']},
 hi:{screens:'तीन नेटिव इंटरफ़ेस',note:'सभी चित्र संस्करण 1.0.1 का अंग्रेज़ी इंटरफ़ेस दिखाते हैं। हर अनूदित README में यही तीन चित्र हैं।',features:'सुविधाएँ',develop:'विकास',bullets:[
 'इंपोर्ट पुराना संस्करण बदल देता है। मूल डाउनलोड सुरक्षित रहता है और पिछली स्थापना कूड़ेदान से वापस लाई जा सकती है।',
 'ऐप आइकन और AppImage के लिए “इसके साथ खोलें” एकीकरण। `--repair-registration` पंजीकरण ठीक करता है।',
 'अनाम HTTPS से सीधे Release डाउनलोड, आर्किटेक्चर और SHA-256 जाँच, वास्तविक प्रगति, प्रतिशत और स्थानांतरित आकार। `gh` या टोकन की आवश्यकता नहीं है।',
 'Ely/GPUI, Qt 6 Widgets और GNOME/GTK 4 में खिसकने वाले विभाजक और सहेजी गई प्राथमिकताएँ; 26 भाषाओं में ऐप का नाम।',
 'AppImage को चलाए या माउंट किए बिना मेटाडेटा और आइकन पढ़ता है।']},
 bn:{screens:'তিনটি নেটিভ ইন্টারফেস',note:'সব ছবিতে 1.0.1 সংস্করণের ইংরেজি ইন্টারফেস দেখানো হয়েছে। প্রতিটি অনূদিত README একই তিনটি ছবি ব্যবহার করে।',features:'বৈশিষ্ট্য',develop:'উন্নয়ন',bullets:[
 'আমদানি পুরোনো সংস্করণ প্রতিস্থাপন করে। মূল ডাউনলোড রাখা হয় এবং আগের ইনস্টলেশন ট্র্যাশ থেকে ফেরানো যায়।',
 'অ্যাপের আইকন ও AppImage-এর “অন্য অ্যাপে খুলুন” সংযোগ। `--repair-registration` নিবন্ধন মেরামত করে।',
 'বেনামি HTTPS দিয়ে সরাসরি Release ডাউনলোড, আর্কিটেকচার ও SHA-256 পরীক্ষা, প্রকৃত অগ্রগতি, শতাংশ ও স্থানান্তরিত আকার। `gh` বা টোকেন লাগে না।',
 'Ely/GPUI, Qt 6 Widgets ও GNOME/GTK 4-এ টেনে সরানো বিভাজক ও সংরক্ষিত পছন্দ; ২৬ ভাষায় প্রদর্শিত অ্যাপের নাম।',
 'AppImage চালানো বা মাউন্ট করা ছাড়াই মেটাডেটা ও আইকন পড়ে।']},
 fa:{screens:'سه رابط بومی',note:'همهٔ تصاویر رابط انگلیسی نسخهٔ 1.0.1 را نشان می‌دهند. تمام ترجمه‌های README از همین سه تصویر استفاده می‌کنند.',features:'امکانات',develop:'توسعه',bullets:[
 'وارد کردن نسخهٔ جدید جایگزین نسخهٔ قدیمی می‌شود. فایل اصلی دانلود حفظ می‌شود و نصب قبلی از سطل زباله قابل بازیابی است.',
 'آیکون برنامه و یکپارچه‌سازی AppImage با «باز کردن با». دستور `--repair-registration` ثبت برنامه را ترمیم می‌کند.',
 'دانلود مستقیم Release از طریق HTTPS ناشناس، بررسی معماری و SHA-256 و نمایش پیشرفت واقعی، درصد و حجم انتقال. نیازی به `gh` یا توکن نیست.',
 'جداکننده‌های قابل کشیدن و تنظیمات ماندگار در Ely/GPUI،‏ Qt 6 Widgets و GNOME/GTK 4؛ نام نمایشی به ۲۶ زبان ترجمه شده است.',
 'خواندن فراداده و آیکون‌ها بدون اجرا یا سوار کردن AppImage.']},
 he:{screens:'שלושה ממשקים מקוריים',note:'כל התמונות מציגות את הממשק האנגלי של גרסה 1.0.1. כל תרגומי README משתמשים באותן שלוש תמונות.',features:'תכונות',develop:'פיתוח',bullets:[
 'ייבוא מחליף גרסאות ישנות. קובץ ההורדה המקורי נשמר וניתן לשחזר התקנות קודמות מסל המחזור.',
 'סמלי יישומים ושילוב AppImage בתפריט „פתיחה באמצעות”. הפקודה `--repair-registration` מתקנת את הרישום.',
 'הורדה ישירה של Releases באמצעות HTTPS אנונימי, בדיקת ארכיטקטורה ו-SHA-256, התקדמות אמיתית, אחוזים וגודל שהועבר. ללא `gh` או אסימון גישה.',
 'מחיצות נגררות והעדפות שמורות ב-Ely/GPUI, ב-Qt 6 Widgets וב-GNOME/GTK 4; שמות תצוגה ב-26 שפות.',
 'קריאת מטא-נתונים וסמלים בלי להפעיל או לעגן את AppImage.']},
 sv:{screens:'Tre inbyggda gränssnitt',note:'Alla bilder visar det engelska gränssnittet i version 1.0.1. Varje översatt README använder samma tre bilder.',features:'Funktioner',develop:'Utveckling',bullets:[
 'Import ersätter äldre versioner. Originalhämtningen behålls och tidigare installationer kan återställas från papperskorgen.',
 'Programikoner och AppImage-integrering för ”Öppna med”. `--repair-registration` reparerar registreringen.',
 'Direkta anonyma HTTPS-hämtningar av Releases med arkitektur- och SHA-256-kontroll, verkligt förlopp, procent och överförd storlek. Inget `gh` eller token krävs.',
 'Dragbara avdelare och sparade inställningar i Ely/GPUI, Qt 6 Widgets och GNOME/GTK 4; visningsnamn på 26 språk.',
 'Läser metadata och ikoner utan att köra eller montera AppImage.']},
 cs:{screens:'Tři nativní rozhraní',note:'Všechny obrázky ukazují anglické rozhraní verze 1.0.1. Každý překlad README používá stejné tři snímky.',features:'Funkce',develop:'Vývoj',bullets:[
 'Import nahrazuje starší verze. Původní stažený soubor zůstává zachován a předchozí instalace lze obnovit z koše.',
 'Ikony aplikací a integrace AppImage do nabídky „Otevřít pomocí“. `--repair-registration` opraví registraci.',
 'Přímé anonymní stahování Releases přes HTTPS s kontrolou architektury a SHA-256, skutečným průběhem, procenty a přenesenou velikostí. Bez `gh` a tokenu.',
 'Přetahovatelné oddělovače a uložená nastavení v Ely/GPUI, Qt 6 Widgets a GNOME/GTK 4; názvy ve 26 jazycích.',
 'Čtení metadat a ikon bez spuštění nebo připojení AppImage.']},
 ro:{screens:'Trei interfețe native',note:'Toate imaginile arată interfața în engleză a versiunii 1.0.1. Fiecare README tradus folosește aceleași trei capturi.',features:'Funcționalități',develop:'Dezvoltare',bullets:[
 'Importul înlocuiește versiunile vechi. Descărcarea originală rămâne, iar instalările anterioare pot fi restaurate din coș.',
 'Pictograme și integrarea AppImage în „Deschide cu”. `--repair-registration` repară înregistrarea.',
 'Descărcări directe de Releases prin HTTPS anonim, cu verificarea arhitecturii și SHA-256, progres real, procent și volum transferat. Fără `gh` sau token.',
 'Separatoare glisabile și preferințe salvate în Ely/GPUI, Qt 6 Widgets și GNOME/GTK 4; nume afișate în 26 de limbi.',
 'Citește metadatele și pictogramele fără a executa sau monta AppImage.']}
};
const file=(locale:string)=>locale==='en'?'README.md':`README.${locale}.md`;
const links=locales.map(([locale,label])=>`[${label}](${file(locale)})`).join(' · ');
const badge='[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)';
const commands='```sh\nbun run scripts/check-i18n.ts\ncargo fmt --all --check\ncargo clippy --all-targets --locked -- -D warnings\ncargo test --release --locked\ncargo build --release --locked\nbun run scripts/package.ts --formats deb\n```';
for(const [locale] of locales){
 const text=copy[locale];if(!text)throw Error('Missing README translation: '+locale);
 const catalog=JSON.parse(readFileSync(`resources/i18n/${locale}.json`,'utf8'));
 const guide=readFileSync(`docs/i18n/${locale}.md`,'utf8');
 let intro=guide.slice(guide.indexOf('\n\n')+2,guide.indexOf('\n## ')).trim();
 intro=intro.replace(/^AppimageMannger/,catalog["AppimageMannger 管理器"]);
 const rtlIntros:Record<string,string>={ar:"مدير AppImage تطبيق أصلي لإدارة تطبيقات AppImage على Linux. اختر تطبيقًا من المكتبة لعرض الأيقونة والإصدار والمعمارية والحجم ومسار التثبيت. ينسخ الاستيراد الملف إلى المجلد المُدار مع إبقاء التنزيل الأصلي.",he:"מנהל AppImage הוא מנהל מקומי ליישומי AppImage ב-Linux. בחרו יישום בספרייה כדי לראות את הסמל, הגרסה, הארכיטקטורה, הגודל ומיקום ההתקנה. הייבוא מעתיק את הקובץ לתיקייה המנוהלת ושומר את ההורדה המקורית.",fa:"مدیر AppImage یک برنامهٔ بومی Linux برای مدیریت AppImage است. با انتخاب برنامه در کتابخانه، آیکون، نسخه، معماری، اندازه و مسیر نصب نمایش داده می‌شود. وارد کردن فایل را به پوشهٔ مدیریت‌شده کپی می‌کند و دانلود اصلی را نگه می‌دارد."};
 intro=rtlIntros[locale]??intro;
 if(locale.startsWith("zh-"))intro=intro.replace(catalog["AppimageMannger 管理器"]+" ",catalog["AppimageMannger 管理器"]);
 const sections=guide.slice(guide.indexOf('\n## ')).trim();
 const licenseAt=sections.lastIndexOf('\n## ');
 const content=[`# ${catalog['AppimageMannger 管理器']}`,badge,links,intro,
  `## ${text.screens}`,text.note,
  '### Ely / GPUI','![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)',
  '### Qt 6 Widgets','![Qt 6 Widgets — English interface](docs/images/qt-en.png)',
  '### GNOME / GTK 4 / libadwaita','![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)',
  `## ${text.features}`,text.bullets.map(b=>'- '+b).join('\n'),
  sections.slice(0,licenseAt),`## ${text.develop}`,
  '[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)',
  commands, sections.slice(licenseAt).trim(),
  'AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)'];
 writeFileSync(file(locale),content.join('\n\n')+'\n');
}
console.log('Generated 26 localized READMEs with shared English screenshots.');
