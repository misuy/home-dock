<template>
    <div class="file_area">
        <button class="download_btn" @click="save_file">download</button>
        <div class="file_name">{{ file.name }}</div>
    </div>
</template>

<script lang="ts">
import { File, save_file_by_url } from '@/util';
import { defineComponent } from 'vue';

export default defineComponent({
    props: {
        input_file: {
            type: File,
            required: true,
        }
    },
    data() {
        return {
            file: this.input_file,
        }
    },
    methods: {
        save_file() {
            this.file.use_url((url) => save_file_by_url(url, this.file.name));
        }
    },
    created() {
        this.file.load_data();
    },
})
</script>

<style>
    .file_area {
        width: 100%;
        height: 100%;
    }
</style>