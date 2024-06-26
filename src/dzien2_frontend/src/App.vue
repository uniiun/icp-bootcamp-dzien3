<!-- główny plik aplikacji vue -->
<!-- część skryptowa -->
<script setup>                                             // jest automatycznie wykonywany w kontekście setup() metody komponentu, co oznacza, że nie musisz jawnie deklarować setup().
                                                           // które mogą być używane do śledzenia i reagowania na zmiany danych w komponentach Vue.
import Blog from './components/Blog.vue';                  //imprort componentu bloga
</script>

<!-- templataka strony głównej -->
<template>
  <main class="container mx-auto">
    <img src="/logo2.svg" alt="DFINITY logo" /> 
    <br />
    <br />

    <Blog />  <!-- wyświetlenie componentu -->
    
  </main>
</template>
