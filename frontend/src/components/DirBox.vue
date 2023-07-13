<template>
    <div class="dir_box" @click="emit_change_path()">
        <img class="entry_img" src="../../public/dir_img.png" height="30"/>
        <div class="entry_name">{{ dir.name }}</div>
    </div>
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
    },
    emits: ["change-path"],
    methods: {
        emit_change_path() {
            this.$emit("change-path", this.dir.path);
        }
    }
})
</script>

<style>
    .entry_name {
        text-align: center;
    }
</style>

<style scoped>
    .dir_box {
        height: 100%;
        display: grid;
        grid-template-columns: min-content auto;
        align-items: center;
    }

    .dir_box:hover {
        background-color: #E4CEF2;
    }
</style>