<template>
  <div class="landing">
    <div class="landing-container">
      <div class="header">
        <span>Hi, I'm <span class="underline">Dytrich</span>.</span>
      </div>
      <div class="gradient">
        I <span class="reveal">code</span>, make
        <span class="reveal">music</span>, and take
        <span class="reveal">pictures</span>.
      </div>
      <router-link to="/about"
        ><div class="learn-more">Learn more</div></router-link
      >
    </div>
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-property-decorator";
export default class Landing extends Vue {}
</script>
<style scoped lang="scss">
.landing {
  font-family: "Lato", sans-serif;
  letter-spacing: 2px;
  font-size: 2rem;
}

.header {
  margin-bottom: 0.5em;
}

.landing-container {
  display: inline-block;
  padding: 2rem;
  background: white;
  text-align: left;

  position: relative;

  left: 50%;
  // For use with sidebar
  //transform: translateX(-50%) translateX(-11rem) translateY(25vh);

  // For no sidebar
  transform: translateX(-50%) translateY(24vh);

  // box-shadow: 0px 0px 20px rgba(0, 0, 0, 0.7);
}

.learn-more {
  text-align: center;
  font-size: 1.7rem;

  width: 30%;
  padding: 0.5rem 0.5rem;
  margin: 1em auto 0 auto;
  background: #2c3e50;
  color: white;

  border: 2px solid #2c3e50;
}

a {
  text-decoration: none;

  transition: opacity 0.3s;

  &:visited {
    color: #2c3e50;
  }

  &:hover {
    opacity: 0.7;
  }
}

.underline {
  transition: all 0.3s ease-in-out;
  &::after {
    z-index: -1;
    content: "";
    position: absolute;
    height: 3px;
    margin: 5px 0 0;

    transition: all 0.3s ease-in-out;
    background: #f27121;
    opacity: 0.5;
    border-radius: 10px;

    /* underline anim */
    top: 2em;
    left: 4.57em;
    width: 0;
    animation-name: slide;
    animation-duration: 4s;
    animation-iteration-count: infinite;

    /* one time
    top: 2em;
    left: 4.57em;
    animation: 0.7s slide 1.5s 1;
    animation-fill-mode: forwards;
    */
  }
}

.gradient {
  background: linear-gradient(
    135deg,
    #8a2387,
    #e94057,
    #f27121,
    #e94057,
    #8a2387
  );
  -webkit-background-clip: text;
  background-size: 200% auto;

  animation: gslide 20s infinite;

  .reveal {
    -webkit-text-fill-color: transparent;
  }
}

@keyframes slide {
  0% {
    width: 0;
    left: 4.57em;
  }
  25% {
    width: 3.65em;
    left: 4.57em;
  }
  50% {
    left: calc(4.57em + 3.65em);
    width: 0;
  }
}
/* one time
@keyframes slide {
  from {
    width: 0;
  }
  to {
    width: 3.65em;
  }
}
*/

@keyframes gslide {
  to {
    background-position: 200% center;
  }
}
</style>
