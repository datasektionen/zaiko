<template>
  <div>
    <p>{{item.name}}</p>
    <p>{{item.location}}</p>
    <p>{{item.current}}</p>
    <input v-model.number="input" />
    <p :class="diffClass">{{input - item.current}}</p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';

const props = defineProps({
  item: {
    name: String,
    current: Number,
  },
});

const input = ref(props.item.current)


const diffClass = computed(() => {
  const diff = input.value - props.item.current
  if (diff >= 0) {
    return "Green"
  } else {
    return "Red"
  }
})

</script>

<style scoped>
  * {
    font-family: Lato;
  }

  div {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
  }

  p,a {
    border-right: 1px solid lightgray;
    padding: 1rem;
  }

  .Red {
    color: red;
  }

  .Green {
    color: green;
  }

  input {
    width: 60%;
  }

  button {
    padding: 1rem;
  }
</style>
