<template>
    <button @click="add_entry_test">click</button>
    <EntryBox v-for="/* eslint-disable-line */ entry in entries" :entry="entry"/>
</template>

<script lang="ts">
    import EntryBox from './EntryBox.vue';
    import { defineComponent } from 'vue';
    import { StorageEntry, EntryType } from "../util"
    import { api_call_read_dir } from "../api_util"

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
            }
        },
        methods: {
            set_entries(entries: Array<StorageEntry>) {
                console.log(entries);
                this.entries = entries;
            },
            add_entry(entry: StorageEntry) {
                this.entries.push(entry);
            },
            add_entry_test() {
                this.add_entry(new StorageEntry("test/path", EntryType.File));
            }
        },
        created() {
            api_call_read_dir(this.path, this.set_entries);
        },
    })
</script>