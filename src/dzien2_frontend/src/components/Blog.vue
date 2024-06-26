<template>                                                  <!-- templatka komponentu vue -->
    <div>
        <h2 class="text-blue-600">Wpisy na bloga</h2>
        <div class="w-100 flex flex-row-reverse">
            <button @click="pobierzWpisy" class="bg-blue-600 rounded text-white p-4">odswiez</button>      <!-- przycisk uruchomiający metode pobierzWpis() -->
        </div>
        elo         <!-- losowy napis -->
        <div class="grid mx-6 gap-4 my-4">                                        
            <div v-for="(wpis, index) in wpisy" class="drop-shadow-xl bg-stone-400 p-4">
                <p>id: {{ index }}</p>
                <p>{{ wpis }}</p>
                <button class="bg-red-600 rounded text-white p-4" @click="usunWpis(index)">usuń</button>
            </div>
        </div>
        <div class="flex justify-content-center flex-col">
            <input v-model="nowyBlog" type="text" class="border-3 border-blue-600 p-4 drop-shadow-md" >              <!-- wiazanie dwukierunkowe z danymi (nowyBlog) i wprowadzenie tekstu -->
            <button @click="dodajWpis" class="bg-green-600 rounded text-white p-4">dodaj</button>
        </div>
    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';
    export default {
        data() {                                                         //dane do których bedziemy sie odnosic
            return {
                wpisy:[],                                                //wektor/tablica wpisów
                nowyBlog: "" ,                                           //wpis
            }
        },
        methods: {                                                       //metody do wykonywania operacji
            async pobierzWpisy(){
                this.wpisy = await dzien2_backend.oczytaj_wpisy();       //await jest po to żeby blockchain nie czekał na odpowiedz (asycnroniczne)
            },
            async dodajWpis(){
               await dzien2_backend.dodaj_wpis(this.nowyBlog);
            },
            async usunWpis(index){
                this.wpisy = await dzien2_backend.usun_wpis(index);
                await this.pobierzWpisy();
            },
        },
        async mounted() {
            this.pobierzWpisy()
        },
    }
</script>