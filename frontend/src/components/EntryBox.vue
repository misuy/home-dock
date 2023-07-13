<template>
    <div class="entry_box">
        <DirBox @change-path="(new_path: String) => emit_change_path(new_path)" v-if="entry?.is_dir()" :dir="cast_entry_to_dir()"/>
        <FileBox v-else :file="cast_entry_to_file()"/>
    </div>
</template>


<script lang="ts">
import { defineComponent } from "vue"
import FileBox from "./FileBox.vue"
import DirBox from "./DirBox.vue"
import { File, Dir, StorageEntry } from "../util"

export default defineComponent  ({
    name: "EntryBox",
    components: {
        FileBox,
        DirBox,
    },
    props: {
        entry: {
            type: StorageEntry,
            required: true,
        }
    },
    emits: ["change-path"],
    methods: {
        cast_entry_to_file(): File {
            return new File(this.entry);
        },

        cast_entry_to_dir(): Dir {
            return new Dir(this.entry);
        },

        emit_change_path(new_path: String) {
            this.$emit("change-path", new_path);
        }
    },
})
</script>

<style>
    .entry_box {
        width: 100%;
        height: 30px;
    }

</style>