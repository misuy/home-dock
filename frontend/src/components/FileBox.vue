<template>
    <div class="file_box">
        <div class="entry_img">
        </div>
        <div class="entry_name">{{ file.name }}</div>
    </div>
</template>


<script lang="ts">
import { defineComponent } from 'vue';
import { File } from "../util"

export default defineComponent ({
    name: "FileBox",
    props: {
        file: {
            type: File,
            required: true
        },
    }
})
</script>

<style scoped>
    .file_box {
        height: 100%;
        display: grid;
        grid-template-columns: min-content auto;
        align-items: center;
    }

    .file_box:hover {
        background-color: #E4CEF2;
    }
</style>