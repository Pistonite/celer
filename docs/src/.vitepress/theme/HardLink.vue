<script setup>
import { ref } from "vue";

const props = defineProps(["link", "text"]);
</script>

<template>
    <a :href="link">{{ text }}</a>
</template>
