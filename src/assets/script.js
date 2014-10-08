(function() {
  var byId = function(x) { return document.getElementById(x); };
  var input = byId("input"),
      errors = byId("errors"),
      primes = byId("prime-list"),
      not_primes = byId("not-prime-list");

  var numbers = {};

  var show_error = function(text) {
    errors.textContent = text;
  }

  var load_number = function(val) {
    var cached = numbers[val];
    if (cached) {
      errors.textContent = '';

      var list = cached.parentNode;
      list.removeChild(cached);
      list.insertBefore(cached, list.firstChild);
      return;
    }

    var req = new XMLHttpRequest();

    req.addEventListener("load", function() {
      errors.textContent = '';

      var data;
      try {
        data = JSON.parse(this.responseText);
      } catch (e if e instanceof SyntaxError) {
        show_error("invalid response for " + val + ": " + this.responseText);
        return;
      }

      if (data.error) {
        show_error(val + ": " + data.error);
      } else {
        var list = data.is_prime ? primes : not_primes;

        var elem = document.createElement("li");
        elem.textContent = val;
        list.insertBefore(elem, list.firstChild);
        numbers[val] = elem;
      }
    });
    req.addEventListener("error", function() {
      errors.textContent = '';
      show_error("could not fetch " + val);
    });

    req.open("GET", "/is_prime/" + encodeURI(val), true);
    req.send();
  };

  var tick = 0;

  input.addEventListener("input", function() {
    var val = input.value;
    if (!val) {
      errors.textContent = '';

      return;
    }

    tick++;
    var my_tick = tick;
    window.setTimeout(function() {
      if (tick == my_tick) {
        load_number(val);
      }
    }, 200);
  });
})()
