<template>
    <div class="entry_name">{{ file.name }}</div>
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