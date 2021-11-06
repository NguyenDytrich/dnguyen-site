import { createApp } from 'https://unpkg.com/petite-vue?module';

let intent_id = '';
let elements = undefined;
let stripe = undefined;

//Initialize Stripe
fetch('/api/tipjar', {
	method: 'POST',
}).then(res => res.json())
	.then(data => {
		stripe = Stripe(data.public_key);
		const clientSecret = data.client_secret;
		intent_id = data.intent_id;

		const options = {
			clientSecret
		};

		elements = stripe.elements(options);
		const paymentElement = elements.create('payment');
		paymentElement.mount('#payment-element');
	});

createApp({
	$delimiters: ['${', '}'],
	tipAmount: 1,
	customAmount: 1,
	tipAmntErr: false,
	validateAmount(e) {
		const regexp = /^([0-9]{1,3},([0-9]{3},)*[0-9]{3}|[0-9]+)(.[0-9][0-9])?$/
		this.tipAmntErr = !regexp.test(e.target.value);
		this.customAmount = Number.parseFloat(e.target.value);
		if(!this.tipAmntErr) {
			this.updateAmount()
		}
	},
	async updateAmount() {
		let amount = 0
		if(this.tipAmount > 0) {
			amount = this.tipAmount * 100;
		} else if (this.tipAmount == -1) {
			amount = this.customAmount * 100;
		}

		await fetch('/api/tipjar', {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				intent_id: intent_id,
				amount: amount,
			})
		});
	},
	async submit(e) {
		e.preventDefault();
		const { error } = await stripe.confirmPayment({
			elements,
			confirmParams: {
				return_url: "http://localhost:8000/redirect/payment",
			}
		});
	}
}).mount()
