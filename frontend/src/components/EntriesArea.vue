<template>
    <div class="entries_area">
        <EntryBox v-for="/* eslint-disable-line */ entry in entries" :entry="entry" @change-path="emit_change_path"/>
    </div>
</template>

<script lang="ts">
import EntryBox from './EntryBox.vue';
import { defineComponent } from 'vue';
import { StorageEntry } from "../util"
import { api_call_read_dir, api_call_read_file } from "../api_util"

export default defineComponent({
    name: "EntriesArea",
    components: {
        EntryBox,
    },
    props: {
        path: {
            type: String,
            required: true,
        }
    },
    data() {
        return {
            entries: new Array<StorageEntry>(),
            mb_img: "",
        }
    },
    emits: ["change-path"],
    methods: {
        set_entries(entries: Array<StorageEntry>) {
            console.log(entries);
            this.entries = entries;
        },
        add_entry(entry: StorageEntry) {
            this.entries.push(entry);
        },
        emit_change_path(new_path: String) {
            this.$emit("change-path", new_path);
        },
    },
    created() {
        api_call_read_dir(this.path, this.set_entries);
    },
})
</script>

<style scoped>
    .entries_area {
        width: 500px;
    }
</style>