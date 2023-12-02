<script setup>
import { computed } from "vue";

const props = defineProps(["name"]);
</script>

<template>
    <i class="fluent-icon" :data-icon="name" aria-hidden="true" ></i>
</template>
