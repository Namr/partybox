<template>
  <div>
    <template v-if="connected == false">
      <Register @wsGotten="establishWS" />
    </template>

    <template v-else>
      <template v-if="inRoom == false">
        <p>Room Select</p>
      </template>
      
      <template v-else>
        <p>The game</p>
      </template>
    </template>
  </div>
</template>

<script>
import Register from "../components/Register.vue";
export default {
  name: "Bang",
  components: {
    Register,
  },
  props: {},
  methods: {
    bangStateMachine(event) {
      console.log(event.data);
    },
    establishWS(url) {
      const socket = new WebSocket("ws:/localhost:8000/ws/" + url);

      this.id = url;

      socket.addEventListener("open", () => {
        this.connected = true;
        socket.send("BANG PLAY");
        socket.send("BANG CREATE niceroom");
        socket.send("BANG LISTROOMS");
      });

      socket.addEventListener("message", this.bangStateMachine);
    },
  },
  data() {
    return {
      id: String,
      connected: false,
      inRoom: false,
    };
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
