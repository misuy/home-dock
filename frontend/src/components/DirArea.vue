<template>
    <div class="dir_area">
        <button class="new_button">new</button>
        <EntryBox v-for="/* eslint-disable-line */ entry in dir.entries" :entry="entry" @change-path="emit_change_path"/>
    </div>
</template>

<script lang="ts">
import EntryBox from './EntryBox.vue';
import { defineComponent } from 'vue';
import { Dir } from "../util"

export default defineComponent({
    name: "DirArea",
    components: {
        EntryBox,
    },
    props: {
        input_dir: {
            type: Dir,
            required: true,
        }
    },
    data() {
        return {
            dir: this.input_dir,
        }
    },
    emits: ["change-path"],
    methods: {
        emit_change_path(new_path: String) {
            this.$emit("change-path", new_path);
        },
        create_new_dir(name: String) {

        },
    },
    created() {
        this.dir.load_entries();
    },
})
</script>

<style scoped>
    .dir_area {
        width: 100%;
    }
</style>