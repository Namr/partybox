<template>
  <div class="container mt-4">
    <div class="row">
      <div class="col-auto mx-auto">
        <b-card title="Bang!" style="max-width: 20rem" class="mb-2 mx-auto">
          <b-card-img :src="require('../assets/bang.jpg')" />
          <b-card-text>
            Bang! is a Wild West-themed social deduction card game. Test your
            deduction skills against others online!
          </b-card-text>

          <b-button to="/bang" variant="primary">Play Online Now</b-button>
        </b-card>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "Home",
  components: {},
  props: {},
  methods: {},
  data() {
    return {};
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
