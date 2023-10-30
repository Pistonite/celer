<script setup>
import { computed } from "vue";

const props = defineProps(["name"]);

const link = computed(() => {
    const sizeIndex = props.name.search(/\d+/g);
    if (sizeIndex === -1) {
        // error case, fallback
        console.error("Invalid icon name: " + props.name);
        return props.name + ".svg";
    }
    const modifierIndex = props.name.substring(sizeIndex).search(/(Regular|Filled)/g);
    if (modifierIndex === -1) {
        // error case, fallback
        console.error("Invalid icon name: " + props.name);
        return props.name + ".svg";
    }
    const noSpacePascal = props.name.substring(0, sizeIndex);
    const size = props.name.substring(sizeIndex, sizeIndex+modifierIndex);
    const modifier = props.name.substring(sizeIndex+modifierIndex);
    console.log(noSpacePascal, size, modifier);

    const spacedPascal = noSpacePascal.replace(/([A-Z])/g, " $1").trim();
    const snake = spacedPascal.replace(/ /g, "_").toLowerCase();
    const urlEncoded = encodeURIComponent(spacedPascal);

    const file = `ic_fluent_${snake}_${size}_${modifier.toLowerCase()}`;
    return `https://raw.githubusercontent.com/microsoft/fluentui-system-icons/main/assets/${urlEncoded}/SVG/${file}.svg`;
});
</script>

<template>
    <i class="fluent-icon" :data-icon="name" aria-hidden="true" ></i>
</template>
