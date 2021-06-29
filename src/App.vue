<template>
  <div id="app">

    <b-navbar toggleable="lg" type="dark" variant="info">
      <b-navbar-brand to="/">Gulag Goons Party Box</b-navbar-brand>
    </b-navbar>

    <router-view />
  </div>
</template>

<script>
export default {
  name: 'App',
  components: {
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 0px;
  box-sizing: content-box;
}
</style>
