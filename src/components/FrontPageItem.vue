<template>
  <div>
    <p>{{item.name}}</p>
    <p>{{item.location}}</p>
    <p>{{item.club}}</p>
    <p>{{item.min}}</p>
    <p>{{item.max}}</p>
    <p>{{item.current}}</p>
    <p>{{item.supplier}}</p>
    <a :href="item.link">Link</a>
    <button>Edit</button>
  </div>
</template>

<script setup lang="ts">
defineProps({
  item: {
    name: String,
    location: String,
    club: String,
    min: Number,
    max: Number,
    current: Number,
    supplier: String,
    link: String,
  },
});

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

  button {
    padding: 1rem;
  }
</style>
