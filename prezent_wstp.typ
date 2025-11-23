#set page(
  background: {
    place(dy:0%)[#image("AGH-title.png",fit:"cover", width: 100%, height: auto)]
  },
  paper:"presentation-4-3",
)
#set text(size:25pt,lang:"pl")
#set align(alignment.horizon)
#set align(alignment.center)

= Gołębie na godzinę (tytuł tymczasowy)

#pagebreak()

#set page(
  background: {
    place(dy:0%)[#image("AGH-slide.png",fit:"cover", width: 100%, height: 100%)]
  },
  paper:"presentation-4-3",
)
#set align(alignment.horizon)
#set align(alignment.left)
#set text(size: 14pt)

= Dane organizacyjne

== Skład zespołu
#let role(a) = text(fill: luma(40%))[#a]

- Piotr Kucia #role([Ui, implementacja wizualizacji danych.])
- Paweł skrzypczyk #role([Backend, implentacja algorytmów, Prace nad symulacją.])
- Patryk Kuszyński #role([UI, Projekt kamery, przesył danych.])

== Cele i założenia projektu:

Celem projektu jest swtorzenie systemu umożliwiającego optymalizację komunikacji miejskich. Jego zadaniem będzie zbieranie i analizowanie danych o ruchu pasażerów na poszczególnych przystankach.

Aby to osiągnąć system będzie przechwytywać obraz z kamery i analizować go pod kątem liczby pasażerów czekających na przystankach w zależności od godziny i rozkładu jazd.

Zebrane dane umożliwią stworzenie modeli predykcyjnych pozwalających na optymalizację rozkładów poszczególnych lini.

Dzięki przejrzystej wizualizacji danych możliwe będzie dostrzegnięcie odpowiednich wzorców gołym okiem. 

#pagebreak()

= Działanie

== Zbieranie danych

Dane do analizy pochodzić będą z kamery umieszczonej na przystanku.
W przypadku problemów z uzyskaniem zgody na nagrywanie przystanków, kamera uchwycać będzie obraz symulacji działającej w środowisku komputerowym.

== Część analityczna

Analiza danych będzie opierała się na algorytmach sztucznej inteligencji działających w np. PyTorch.
Ich zadaniem będzie wyciągniecie z obrazu danych takich jak liczba osób / nadjeżdzające autobusy oraz predykcją na podstawie tych danych ilości przyszłych pasażerów na przystanku.

== Wizualizacja danych

+ Podgląd na żywo z kamery (kiedy kamera jest online).
+ Panel danych analitycznych:
  - Średnia ilość osób na godzine, pokazana na wykresie.
  - Przewidywana ilość osób w określonych godzinach, również na wykresie.
  - Sukcess predykcji dla określonych godzin.
+ Panel logów analitycznych.
  - Sygnały sytemowe.
  - Komunikaty o działaniach podejmowanych przez system.

#pagebreak()

#align(alignment.center)[
#stack(dir: ltr, spacing:10%,
    stack(dir:ttb,spacing:10%,image("./Diagram1.png",width: 50%),role([Źródła wizualizowanych danych])), 
    stack(dir:ttb,spacing:10%,image("./Wizualizacja1.png",width: 50%),role([Szkielet UI])))
]

#pagebreak()

= Plan realizacji projektu:

== Etap 1 (tydzień 1-2):
- Przygotowanie kamery
- Wybór źródła danych
- Rozpoczęcie pracy nad środowiskiem graficznym
== Etap 2 (tydzień 3-5):
- Implementacja systemu detekcji osób na obrazie
- Przygotowanie bazy danych
== Etap 3 (tydzień 6-8):
- Opracowanie systemu predykcyjnego
- Testy i kalibracja programów
== Etap 4:
- Integracja wszystkich modułów
- Testy końcowe, poprawki, dokumentacja

