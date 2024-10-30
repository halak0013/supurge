# Süpürge uygulaması

`cargo add rand`

flowchart TD
    A[Başlangıç Noktası] --> B[Rastgele Nokta Seç]
    B --> C{En Yakın Düğüm Bul}
    C --> D[Yeni Düğüme Doğru\nBelirli Uzaklık İlerle]
    D --> E{Engel Kontrolü}
    E -->|Engel Var| F[Bu Yolu İptal Et\nYeni Rastgele\nNokta Seç]
    F --> B
    E -->|Engel Yok| G[Yeni Düğümü\nAğaca Ekle]
    G --> H{Hedefe Ulaşıldı mı?}
    H -->|Hayır| B
    H -->|Evet| I[Yolu Tamamla]