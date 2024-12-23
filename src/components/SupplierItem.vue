<template>
  <div>
    <p>{{item.name}}</p>
    <p>{{item.username}}</p>
    <p>{{item.password}}</p>
    <a :href="item.link">Link</a>
    <p>{{item.note}}</p>
    <button>Edit</button>
  </div>
</template>

<script setup lang="ts">
defineProps({
  item: {
    name: String,
    username: String,
    password: String,
    link: String,
    note: String,
  },
});

</script>

<style scoped>
  * {
    font-family: Lato;
  }

  div {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr 2fr 1fr;
  }

  p,a {
    border-right: 1px solid lightgray;
    padding: 1rem;
  }

  button {
    padding: 1rem;
  }
</style>
