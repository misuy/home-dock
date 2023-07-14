<template>
    <div class="img_area">
        <button class="download_btn" @click="save_img">download</button>
        <img class="img" :src="img.url" :key="img.url"/>
    </div>
</template>

<script lang="ts">
import { Img, save_file_by_url } from '@/util';
import { defineComponent } from 'vue';

export default defineComponent({
    props: {
        input_img: {
            type: Img,
            required: true,
        }
    },
    data() {
        return {
            img: this.input_img,
        }
    },
    methods: {
        save_img() {
            this.img.use_url((url) => save_file_by_url(url, this.img.name))
        },
    },
    created() {
        this.img.create_url();
    },

})
</script>

<style>
    .img_area {
        width: 100%;
        height: 100%;
        display: grid;
        grid-template-rows: min-content, auto;
    }
    .download_btn {
        grid-row: 1;
    }
    .img {
        grid-row: 2;
        width: 100%;
        height: 100%;
    }
</style>