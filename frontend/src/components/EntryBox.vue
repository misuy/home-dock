<template>
    <div class="entry_box" @click="emit_change_path">
        <div class="entry_img_holder">
            <img class="entry_img" :src="entry_img"/>
        </div>
        <div class="entry_name">{{ entry.name }}</div>
    </div>
</template>


<script lang="ts">
import { defineComponent } from "vue"
import { StorageEntry } from "../util"

export default defineComponent  ({
    name: "EntryBox",
    data() {
        return {
            entry_img: this.get_entry_img(),
        }
    },
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
        },
        get_entry_img(): string {
            if (this.entry.is_file()) return "file_img.png";
            else return "dir_img.png";
        },
    },
})
</script>

<style>
    .entry_box {
        width: 100%;
        height: 30px;
        display: grid;
        grid-template-columns: 10px min-content auto;
        text-align: center;
        align-items: center;
    }

    .entry_box:hover {
        background-color: #8fbeff;
    }

    .entry_img_holder {
        height: 25px;
        grid-column: 2;
    }

    .entry_img {
        height: 100%;
    }
</style>