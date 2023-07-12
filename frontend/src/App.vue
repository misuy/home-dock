<template>
  <EntriesArea path=""/>
  <EntryBox :entry="example_entry()"/>
</template>

<script lang="ts">
import EntryBox from "./components/EntryBox.vue"
import EntriesArea from "./components/EntriesArea.vue";
import { EntryType, StorageEntry } from "./util";

export default {
  name: 'App',
  components: {
    EntryBox,
    EntriesArea,
  },
  methods: {
    example_entry(): StorageEntry {
      return new StorageEntry("hehe/123/hihi", EntryType.Dir);
    }
  }
}
</script>

<style>
#app {
}
</style>
