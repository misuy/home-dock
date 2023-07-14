<template>
    <div class="entry_box" @click="emit_change_path">
        <div class="entry_img">
        </div>
        <div class="entry_name">{{ entry.name }}</div>
    </div>
</template>


<script lang="ts">
import { defineComponent } from "vue"
import { StorageEntry } from "../util"

export default defineComponent  ({
    name: "EntryBox",
    props: {
        entry: {
            type: StorageEntry,
            required: true,
        }
    },
    emits: ["change-path"],
    methods: {
        emit_change_path() {
            this.$emit("change-path", this.entry.path);
        }
    },
})
</script>

<style>
    .entry_box {
        width: 100%;
        height: 30px;
        height: 100%;
        display: grid;
        grid-template-columns: min-content auto;
        align-items: center;
    }

    .entry_box:hover {
        background-color: #E4CEF2;
    }
</style>