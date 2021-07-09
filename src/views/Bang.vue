<template>
  <div>
    <template v-if="connected == false">
      <Register @wsGotten="establishWS" />
    </template>

    <template v-else>
      <template v-if="inRoom == false">
        <h2>Select a Room</h2>
        <div class="container mt-4">
          <div class="row">
            <div class="col-auto mb-3" v-for="room in rooms" :key="room.id">
              <b-card
                :title="room.name"
                style="max-width: 20rem"
                class="mb-2 mx-auto"
              >
                <b-button v-on:click="joinRoom(room.name)" variant="primary">Join Room</b-button>
              </b-card>
            </div>
          </div>
        </div>

        <div>
          <b-card style="max-width: 20rem" class="mb-2 mx-auto">
            <b-form @submit="createRoom">
              <b-form-group
                id="name-input-group"
                label="Create a Room:"
                label-for="input-1"
              >
                <b-form-input
                  id="roomname-input"
                  v-model="roomName"
                  placeholder="Enter a name"
                  required
                ></b-form-input>
              </b-form-group>
              <b-button type="submit" variant="primary">Create</b-button>
            </b-form>
          </b-card>
        </div>
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
      var message = event.data.replace(/\n/g, " ").split(" ");

      if (message[0] === "RESPONSE" && message.length >= 2) {
        //if we are getting a list of rooms add it to our internal datastructure
        if (message[1] === "LISTROOMS") {
          var lines = event.data.split(/\r?\n/);

          for (var i = 1; i < lines.length - 1; i++) {
            const regex = /name: (\w+), id: (\w+), players: (\w+)/g;
            var match = regex.exec(lines[i]);
            this.rooms.push({
              name: match[1],
              id: match[2],
              players: match[3],
            });
          }
        } else if (message[1] === "JOIN") {
          this.inRoom = true;
        } else if (message[1] === "CREATE") {
          this.inRoom = true;
        }
      }

      console.log(event.data);
    },
    establishWS(url) {
      this.socket = new WebSocket("ws:/localhost:8000/ws/" + url);

      this.id = url;

      this.socket.addEventListener("open", () => {
        this.connected = true;
        this.socket.send("BANG PLAY");
        this.socket.send("BANG LISTROOMS");
      });

      this.socket.addEventListener("message", this.bangStateMachine);
    },
    createRoom(event) {
      event.preventDefault();
      this.socket.send("BANG CREATE " + this.roomName);
    },
    joinRoom(name) {
       this.socket.send("BANG JOIN " + name);
    }
  },
  data() {
    return {
      id: String,
      connected: false,
      inRoom: false,
      rooms: [],
      roomName: "",
      socket: WebSocket,
    };
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
