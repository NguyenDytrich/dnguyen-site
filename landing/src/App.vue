<template>
  <div id="app">
    <!--
    <transition appear name="slide-in-left">
      <SideNav v-show="!landing" id="nav" />
    </transition>
    -->
    <div class="router-container">
      <transition name="fade-up">
        <router-view></router-view>
      </transition>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import SideNav from "@/components/SideNav.vue";

@Component({
  components: {
    SideNav
  }
})
export default class App extends Vue {
  get landing() {
    return this.$route.name === "Landing" ? true : false;
  }
}
</script>

<style lang="scss">
@import url("https://fonts.googleapis.com/css2?family=Lato&display=swap");

body {
  margin: 0;
  padding: 0;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  //background-color: #f0ece3;
  //background-color: #222831;
  background: white;
  color: #2c3e50;

  height: 100vh;
  display: grid;
  grid-template-columns: 22rem auto;
  grid-template-rows: auto auto auto;
}

#nav {
  font-family: "Lato", sans-serif;
  letter-spacing: 0.02rem;
  padding-top: 2rem;

  color: white;
  background-color: white;

  grid-column-start: 1;
  grid-column-end: 2;
  grid-row-start: 1;
  grid-row-end: -1;
}

.router-container {
  //grid-column-start: 2;
  //grid-column-end: -1;
  //grid-row-start: 1;
  //grid-row-end: -1;
  grid-area: 1/1/-1/-1;
  padding: 2rem;
}

.landing-container {
  grid-column-start: 1;
}

.fade-up-leave-active {
  transition: all 0.5s ease;
}

.fade-up-enter-active {
  transition: all 0.5s 0.5s ease;
}

.fade-up-enter,
.fade-up-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

.slide-in-left-enter-active {
  transition: all 0.5s 0.5s ease;
}

.slide-in-left-leave-active {
  transition: all 0.5s 0.3s ease;
}

.slide-in-left-enter,
.slide-in-left-leave-to {
  transform: translateX(-100%);
}
</style>
