<template> <!-- templatka komponentu vue -->
    <div>
        <button @click="pobierzWpisy">odswiez</button>  <!-- przycisk uruchomiający metode pobierzWpis() -->
        elo <!-- losowy napis -->
        {{ wpisy }} <!-- wypisywanie wpisów -->
        <input v-model="nowyBlog" type="text">  <!-- wiazanie dwukierunkowe z danymi (nowyBlog) i wprowadzenie tekstu -->
        <button @click="dodajWpis">dodaj</button>
    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';
    export default {
        data() {                //dane do których bedziemy sie odnosic
            return {
                wpisy:[],       //wektor/tablica wpisów
                nowyBlog: "" ,  //wpis
            }
        },
        methods: {              //metody do wykonywania operacji
            async pobierzWpisy(){
                this.wpisy = await dzien2_backend.oczytaj_wpisy();  //await jest po to żeby blockchain nie czekał na odpowiedz (asycnroniczne)
            },
            async dodajWpis(){
               await dzien2_backend.dodaj_wpis(this.nowyBlog);
            }
        },
    }
</script>