<template>
    <div class="entry_name">{{ dir.name }}</div>
</template>


<script lang="ts">
import { defineComponent } from 'vue';
import { Dir } from "../util"

export default defineComponent ({
    name: "DirBox",
    props: {
        dir: {
            type: Dir,
            required: true,
        },
    }
})
</script>