<template>
    <EntriesArea :path="storage_path" :key="storage_path" @change-path="change_path"/>
</template>

<script lang="ts">
import EntriesArea from "./EntriesArea.vue";
import { defineComponent } from "vue";

export default defineComponent({
    name: 'StorageArea',
    components: {
        EntriesArea,
    },
    methods: {
        change_path(new_path: String) {
            this.$router.push({ path: new_path.toString() });
        }
    },
    computed: {
        storage_path() {
            let path = "";
            for (let el of (this.$route.params.storage_path as Array<string>)) {
                path += "/" + el;
            }
            if (path == "") path = "/";
            console.log(path);
            return path;
        }
    },
})
</script>

<style>
    * {
        margin: 0;
        padding: 0;
    }
</style>
