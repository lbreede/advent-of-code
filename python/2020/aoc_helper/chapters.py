import json

chapters = {
	"1": "Report Repair", "2": "Password Philosophy",
	"3": "Toboggan Trajectory", "4": "Passport Processing",
	"5": "Binary Boarding", "6": "Custom Customs",
	"7": "Handy Haversacks", "8": "Handheld Halting",
	"9": "Encoding Error", "10": "Adapter Array",
	"11": "Seating System"
}

with open("chapters.json", "w") as f:
	json.dump(chapters, f, indent=4)