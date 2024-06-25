use std::cell::RefCell;      //RefCell pozwala na mutowalne przechowywanie danych wewnątrz struktury,   
                            //która jest sama niezmienna

thread_local! {         // Makro thread_local! definiuje zmienne, które są specyficzne dla każdego wątku. Oznacza to, że każdy wątek ma własną, odrębną kopię tej        
                        //zmiennej, co zapobiega problemom z dostępem do współdzielonych zasobów z różnych wątków (czyli problemom z synchronizacją)                                 
    static WPISY: RefCell<Vec<String>> = RefCell::default(); //zmienna specyficzna statyczna o nazawie WPISY przechowuje wektor (Vec) ciągów znaków (String). Używa RefCell jako kontenera,
    // który umożliwia mutowanie zawartości wektora. RefCell::default() inicjalizuje RefCell z domyślną wartością, która w przypadku Vec<String> jest pustym wektorem.
}
#[ic_cdk::query]                                   //zapytanie do kanistra przez icp
fn greet(name: String, last_name: i8) -> String {   //funkcja powitalna przyjmujaca argumenty imienia (typu string i nazwiska typu int 8-bit)
    format!("Hello, {} {}!", name, last_name)       //wyświetlenie tekstu sformatowanego przyjmijącą jako wartosc dwie zmienienne
}
#[ic_cdk::update]                                  //wysłanie do kanistra informacji
fn dodaj_wpis(wpis:String){                         //funkcja dodająca wpis przyjmującą wpisy (string)
    WPISY.with(|wpisy|{     //Ta funkcja jest wywoływana z referencją do WPISY specyficznej dla bieżącego wątku.
        wpisy.borrow_mut().push(wpis)               //pozwala na uzyskanie mutowalnej referencji do przechowywanej wartości i dodaje nowy element (ciąg znaków wpis) do wektora.
    })
}
#[ic_cdk::query]                                //zapytanie do kanistra przez icp
fn oczytaj_wpisy()->Vec<String>{                //funkcja wypisująca vector, czyli tablice wpisów
    WPISY.with(|wpisy|{ // sytuacja analogincza jak przy dodawaniu
        wpisy.borrow().clone()              //ale teraz pobieramy wartość niemutowalną i ją klonujemy do wektora
    })
}