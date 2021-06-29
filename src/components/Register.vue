<template>
<div>
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
</div>
</template>

<script>
export default {
  name: "Register",
  props: {},
  methods: {},
  data() {
    return {
      username: String,
    };
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
