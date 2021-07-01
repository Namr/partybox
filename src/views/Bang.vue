<template>
  <div>
    <Register @wsGotten="establishWS" />
  </div>
</template>

<script>
import Register from "../components/Register.vue"
export default {
  name: 'Bang',
  components: {
    Register
  },
  props: {
  },
  methods: {
    establishWS(url) {
      const socket = new WebSocket("ws:/localhost:8000/ws/" + url);

      console.log(url);
      socket.addEventListener('open', function () {
        socket.send('TELL bang! cool stuff!');
      });

      socket.addEventListener('message', function (event) {
        console.log(event.data);
      })
    }
  },
  data() {
    return {
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

</style>
