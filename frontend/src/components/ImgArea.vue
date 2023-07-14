<template>
    <div class="img_area">
        <img class="img" :src="img.url" :key="img.url"/>
    </div>
</template>

<script lang="ts">
import { Img } from '@/util';
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
    created() {
        this.img.create_url();
    },

})
</script>

<style>
    .img_area {
        width: 100%;
        height: 100%;
    }
    .img {
        width: 100%;
        height: 100%;
    }
</style>