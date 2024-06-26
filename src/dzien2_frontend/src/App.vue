<!-- główny plik aplikacji vue -->
<!-- część skryptowa -->
<script setup>  // jest automatycznie wykonywany w kontekście setup() metody komponentu, co oznacza, że nie musisz jawnie deklarować setup().
import { ref } from 'vue';                            // Importuje funkcję ref z biblioteki Vue. ref służy do tworzenia reaktywnych zmiennych,
                                                      // które mogą być używane do śledzenia i reagowania na zmiany danych w komponentach Vue.
import { dzien2_backend } from 'declarations/dzien2_backend/index'; // importuje backend aplikacji
import Blog from './components/Blog.vue';             //imprort componentu bloga

let greeting = ref('');                                   //tworzy reaktywną zmienną greeting, która początkowo przechowuje pusty ciąg znaków

async function handleSubmit(e) {
  e.preventDefault();                                      // Zapobiega domyślnemu zachowaniu formularza (odświeżenie strony)
  const target = e.target;                                 // Pobiera element, który wywołał zdarzenie (formularz)
  const name = target.querySelector('#name').value;        // Pobiera wartość z pola tekstowego o id 'name'
  const numer = target.querySelector('#numer').value;      // Pobiera wartość z pola tekstowego o id 'numer'
  await dzien2_backend.greet(name, Number(numer)).then((response) => {    // Wywołuje asynchronicznie funkcję 'greet' z backendu 
  //                                                                      z dwoma argumentami: name i numer (skonwertowany na liczbę)
    greeting.value = response;                              // Ustawia wartość reaktywnej zmiennej 'greeting' na odpowiedź z backendu
  });
}
</script>
<!-- templataka strony głównej -->
<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" /> 
    <br />
    <br />
    
    <form action="#" @submit="handleSubmit">  <!-- "@" skraca eventy z js  -->
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <input id="numer" alt="Numer" type="number" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section><!-- w {{ wypisuje sie zmienne }} -->
    <Blog />  <!-- wyświetlenie componentu -->
    
  </main>
</template>
