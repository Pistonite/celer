<script setup>
import { ref } from 'vue'

const props = defineProps(["link"])
</script>

<template>
    <code>
        <a :href="'https://github.com/Pistonite/celer/blob/main/' + link">{{ link }}</a>
    </code>
</template>