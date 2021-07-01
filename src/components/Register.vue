<template>
  <div>
    <b-card style="max-width: 20rem" class="mb-2 mx-auto">
      <b-form @submit="onSubmit">
        <b-form-group
          id="name-input-group"
          label="Username:"
          label-for="input-1"
        >
          <b-form-input
            id="username-input"
            v-model="username"
            placeholder="Enter a name"
            required
          ></b-form-input>
        </b-form-group>
        <b-button type="submit" variant="primary">Submit</b-button>
      </b-form>
    </b-card>
  </div>
</template>

<script>
import axios from "axios";
export default {
  name: "Register",
  props: {},
  methods: {
    onSubmit(event) {
      event.preventDefault();

      this.id = this.cyrb53(this.username);

      //get a websocket connection and pass it up the chain
      axios.post("api/register", { user_id: this.id }).then((response) => {
        this.$emit("wsGotten", response.data.url);
      });
    },
    cyrb53(str, seed = 0) {
      let h1 = 0xdeadbeef ^ seed,
        h2 = 0x41c6ce57 ^ seed;
      for (let i = 0, ch; i < str.length; i++) {
        ch = str.charCodeAt(i);
        h1 = Math.imul(h1 ^ ch, 2654435761);
        h2 = Math.imul(h2 ^ ch, 1597334677);
      }
      h1 =
        Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^
        Math.imul(h2 ^ (h2 >>> 13), 3266489909);
      h2 =
        Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^
        Math.imul(h1 ^ (h1 >>> 13), 3266489909);
      return 4294967296 * (2097151 & h2) + (h1 >>> 0);
    },
  },
  data() {
    return {
      username: "",
      id: Number,
    };
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
